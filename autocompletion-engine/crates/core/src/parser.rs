use std::{
  collections::{HashMap, HashSet},
  fs::read_to_string,
};

use itertools::Itertools;
use log::error;
use tree_sitter::{Parser, Query, QueryCursor};

pub type Completions = Vec<(ClassName, RuleSet)>;
type FileClassMap = HashMap<Filename, HashMap<ClassName, RuleSetMap>>;
type Filename = String;
type ClassName = String;
type RuleSet = String;

type RuleSetMap = HashMap<RuleSetId, HelpDoc>;
type RuleSetId = usize;
type HelpDoc = String;

pub struct AutocompletionEngine {
  completions: Completions,
  file_class_map: FileClassMap,
  parser: Parser,
  query: Query,
  query_cursor: QueryCursor,
}

impl AutocompletionEngine {
  pub fn new() -> Self {
    let parser = match build_parser() {
      Ok(p) => p,
      Err(e) => {
        error!("Failed to build parser: {}", e);
        panic!("Failed to build parser ");
      }
    };
    let query = match get_class_selectors_query_for_tree() {
      Some(q) => q,
      None => {
        error!("Failed to create query for tree");
        panic!("Failed to create query for tree");
      }
    };

    Self {
      completions: vec![],
      file_class_map: HashMap::new(),
      parser,
      query,
      query_cursor: QueryCursor::new(),
    }
  }

  pub fn get_all_completions_as_string(&mut self, files: Vec<String>) -> String {
    let completions = self.get_all_completions_for_files(files);
    match serde_json::to_string(completions) {
      Ok(stringified) => stringified,
      Err(e) => {
        error!("Could not convert class hashmap to string: {}", e);
        String::from("")
      }
    }
  }

  pub fn invalidate_cache(&mut self) {
    self.completions = vec![];
  }

  pub fn invalidate_file_cache(&mut self, file_path: String) {
    self.file_class_map.remove(&file_path);
  }

  pub fn get_all_completions_for_files(&mut self, files: Vec<String>) -> &Completions {
    // Ensure we don't operate on the same file twice.
    let unique_files: HashSet<String> = files.into_iter().collect();
    let files_already_processed = unique_files
      .iter()
      .all(|key| self.file_class_map.contains_key(key));

    if files_already_processed {
      return &self.completions;
    }

    for path in unique_files {
      if self.file_class_map.contains_key(&path) {
        continue;
      }

      let mut class_rule_map: HashMap<ClassName, RuleSetMap> = HashMap::new();
      let code = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => {
          error!("Could not read file {}: {}", path, e);
          continue;
        }
      };
      let tree = match self.parser.parse(&code, None) {
        Some(tree) => tree,
        None => {
          error!("Could not parse code in file {}", path);
          continue;
        }
      };
      let code = code.as_bytes();

      let matches = self
        .query_cursor
        .matches(&self.query, tree.root_node(), code);

      for each_match in matches {
        let [class_selector, class_name] = each_match.captures else {
          error!("Could not destructure captures");
          continue;
        };

        let class_selector = class_selector.node;
        let class_name = class_name.node;

        // Walk upwards, finding the parent rule set.
        let mut parent = class_selector.parent();
        while let Some(found_parent) = parent {
          if found_parent.kind() == "rule_set" {
            break;
          } else {
            parent = found_parent.parent();
          }
        }

        let rule_set_node = match parent {
          Some(p) => p,
          None => {
            error!(
              "Could not find parent rule set for: {}. Likely a malformed stylesheet.",
              class_selector
                .utf8_text(code)
                .expect("Could not convert node to utf8 text")
            );
            continue;
          }
        };

        let rule_set = match rule_set_node.utf8_text(code) {
          Ok(rule_set) => rule_set,
          Err(e) => {
            error!(
              "Could not convert node to utf8 text: {}. Error was: {}",
              rule_set_node.kind(),
              e
            );
            continue;
          }
        }
        .to_string();

        let class_name = match class_name.utf8_text(code) {
          Ok(class_name) => class_name,
          Err(e) => {
            error!(
              "Could not convert node to utf8 text: {}. Error was: {}",
              class_name.kind(),
              e
            );
            continue;
          }
        }
        .to_string();

        class_rule_map
          .entry(class_name)
          .and_modify(|rule_map| {
            rule_map
              .entry(rule_set_node.id())
              .or_insert(rule_set.to_owned());
          })
          .or_insert(HashMap::from([(rule_set_node.id(), rule_set)]));

        self
          .file_class_map
          .insert(path.clone(), class_rule_map.clone());
      }
    }

    // Convert the file-centric map into the completions list.
    // I tried a .reserve and .drain approach, and there was no benchmarking
    // improvement. So I opted for the more readable version.
    for class_rule_map in self.file_class_map.values() {
      for (class_name, rule_set_map) in class_rule_map {
        let rule_sets: Vec<String> = rule_set_map.clone().into_values().sorted().collect();
        self
          .completions
          .push((class_name.clone(), rule_sets.join("\n\n")));
      }
    }

    self.completions.sort_unstable();
    &self.completions
  }
}

impl Default for AutocompletionEngine {
  fn default() -> Self {
    Self::new()
  }
}

fn build_parser() -> Result<Parser, &'static str> {
  let mut parser = Parser::new();
  if parser.set_language(tree_sitter_css::language()).is_err() {
    error!("Error loading scss grammar");
    return Err("Error loading scss grammar");
  }
  Ok(parser)
}

fn get_class_selectors_query_for_tree() -> Option<Query> {
  match Query::new(
    tree_sitter_css::language(),
    r#"
            (class_selector
                (class_name) @class_name
            ) @class_selector
        "#,
  ) {
    Ok(query) => Some(query),
    Err(_) => {
      error!("Could not create tree sitter query");
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn get_list() -> [(&'static str, &'static str); 6] {
    [
      (
        "drag-and-drop",
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "focus",
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "focused",
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "input-contrast",
        "input.input-contrast,\n.input-contrast {\n  background-color: #fafafa;\n}\n\ninput.input-contrast:focus,\n.input-contrast:focus {\n  background-color: #fff;\n}"
      ),
      (
        "search-page-label",
        "input[type=\"text\"],\n#adv_code_search .search-page-label,\ntextarea {\n  min-height: 34px;\n  padding: 7px 8px;\n  font-size: 13px;\n  color: #333;\n  vertical-align: middle;\n  background-color: #fff;\n  background-repeat: no-repeat;\n  background-position: right center;\n  border: 1px solid #ccc;\n  border-radius: 3px;\n  outline: none;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075);\n}\n\ninput[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}",
      ),
      (
        "wrapper",
        "#peek .wrapper {\n  width: 860px !important;\n  padding: 0;\n}\n\n#peek2 .wrapper {\n  border: 1px solid red;\n}",
      ),
    ]
  }

  #[test]
  fn file_with_no_classnames_provides_no_completions() {
    let mut engine = AutocompletionEngine::new();
    let completions =
      engine.get_all_completions_for_files(vec!["./__test__/no_classnames.css".to_string()]);
    assert!(completions.is_empty());
  }

  #[test]
  fn can_get_completions() {
    let mut engine = AutocompletionEngine::new();
    let actual = engine.get_all_completions_for_files(vec!["./__test__/basic.css".to_string()]);
    let expected = get_list();

    assert_eq!(actual.len(), expected.len());

    for (class_name, rule_set) in &expected {
      assert!(actual.contains(&(class_name.to_string(), rule_set.to_string())));
    }
  }

  #[test]
  fn can_get_completions_as_string() {
    let mut engine = AutocompletionEngine::new();
    let list = get_list();
    let actual = engine.get_all_completions_as_string(vec!["./__test__/basic.css".to_string()]);
    let expected =
      serde_json::to_string(&list).expect("Could not convert file-centric map to string");
    assert_eq!(actual, expected);
  }

  #[test]
  fn handles_invalid_file_gracefully() {
    let mut engine = AutocompletionEngine::new();
    let completions =
      engine.get_all_completions_for_files(vec!["non_existent_file.css".to_string()]);
    assert!(completions.is_empty());
  }

  #[test]
  fn can_invalidate_cache() {
    let mut engine = AutocompletionEngine::new();
    engine.get_all_completions_for_files(vec!["./__test__/basic.css".to_string()]);
    assert!(!engine.file_class_map.is_empty());
    engine.invalidate_file_cache("./__test__/basic.css".to_string());
    assert!(engine
      .file_class_map
      .get("./__test__/test.atom.io.css")
      .is_none());
  }

  // This isn't perfect, but it's something. Not sure how to properly test this.
  #[test]
  fn uses_cache_for_subsequent_calls() {
    let mut engine = AutocompletionEngine::new();
    let first_call = engine
      .get_all_completions_for_files(vec!["./__test__/basic.css".to_string()])
      .clone();
    let second_call = engine
      .get_all_completions_for_files(vec!["./__test__/basic.css".to_string()])
      .clone();
    assert_eq!(first_call, second_call);
  }

  #[test]
  fn test_file_cache_invalidation() {
    let mut engine = AutocompletionEngine::new();
    engine.get_all_completions_for_files(vec!["./__test__/basic.css".to_string()]);
    assert!(!engine.file_class_map.is_empty());

    engine.invalidate_file_cache("./__test__/basic.css".to_string());
    assert!(engine.file_class_map.get("./__test__/basic.css").is_none());
  }

  #[test]
  fn multiple_files_can_provide_completions() {
    let mut engine = AutocompletionEngine::new();
    engine.get_all_completions_for_files(vec![
      "./__test__/basic.css".to_string(),
      "./__test__/basic2.css".to_string(),
    ]);

    let combined_length = engine.file_class_map["./__test__/basic.css"].len()
      + engine.file_class_map["./__test__/basic2.css"].len();
    assert_eq!(engine.completions.len(), combined_length);
  }

  #[test]
  fn multiple_rulesets_for_single_classname_across_files() {
    let mut engine = AutocompletionEngine::new();
    let completions = engine.get_all_completions_for_files(vec![
      "./__test__/basic.css".to_string(),
      "./__test__/another_file_with_same_classname.css".to_string(),
    ]);

    // Locate the class name in question within the completions list
    let class_entries: Vec<&(String, String)> = completions
      .iter()
      .filter(|(class_name, _)| class_name == "wrapper")
      .collect();

    assert!(class_entries.len() == 2);
  }

  #[test]
  fn duplicate_classnames_across_files() {
    let mut engine = AutocompletionEngine::new();
    engine.get_all_completions_for_files(vec![
      "./__test__/basic.css".to_string(),
      "./__test__/another_file_with_same_classname.css".to_string(),
    ]);

    let duplicate_class_count = engine
      .completions
      .iter()
      .filter(|(class_name, _)| class_name == "wrapper")
      .count();
    assert_eq!(duplicate_class_count, 2);
  }

  #[test]
  fn test_file_with_malformed_css() {
    let mut engine = AutocompletionEngine::new();
    let completions =
      engine.get_all_completions_for_files(vec!["./__test__/malformed.css".to_string()]);

    assert_eq!(
      completions,
      &[(
        "wrapper".to_string(),
        "#peek2 .wrapper {\n  border: 1px solid red;\n}".to_string()
      )]
    );
  }
}
