use std::{
  collections::{HashMap, HashSet},
  fs::read_to_string,
  println,
};

use itertools::Itertools;
use log::error;
use tree_sitter::{Parser, Query, QueryCursor};

pub type Completions = Vec<(ClassName, RuleSet)>;
type IntermediateCompletions = HashMap<ClassName, RuleSetMap>;
type ClassName = String;
type RuleSet = String;

type RuleSetMap = HashMap<RuleSetId, HelpDoc>;
type RuleSetId = usize;
type HelpDoc = String;

pub struct AutocompletionEngine {
  completions: Completions,
  query: Query,
  query_cursor: QueryCursor,
}

impl AutocompletionEngine {
  pub fn new() -> Self {
    let query = match get_class_selectors_query_for_tree() {
      Some(q) => q,
      None => {
        error!("Failed to create query for tree");
        panic!("Failed to create query for tree");
      }
    };

    Self {
      completions: vec![],
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

  // TODO: we're currently structured around classnames. This is nice for looping and
  // creating the final completions list, but makes it harder to invalidate the cache
  // for a single file. Need to rethink this.
  pub fn get_all_completions_for_files(&mut self, files: Vec<String>) -> &Completions {
    if !self.completions.is_empty() {
      return &self.completions;
    }

    // Ensure we don't operate on the same file twice.
    let files: HashSet<String> = files.into_iter().collect();
    let mut rule_maps_by_class_name: IntermediateCompletions = HashMap::new();

    for path in files {
      let code = match read_to_string(&path) {
        Ok(code) => code,
        Err(e) => {
          error!("Could not read file {}: {}", path, e);
          continue;
        }
      };
      // Deciding for now to rebuild the parser for every file since I don't know how it
      // works under the hood. It's probably fine to reuse it, but I don't want to risk
      // some weird state bug.
      let mut parser = match self.build_parser() {
        Ok(p) => p,
        Err(e) => {
          error!("Failed to build parser: {}", e);
          continue;
        }
      };
      let tree = match parser.parse(&code, None) {
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
          println!("Could not destructure captures");
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
            println!(
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
            println!(
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
            println!(
              "Could not convert node to utf8 text: {}. Error was: {}",
              class_name.kind(),
              e
            );
            continue;
          }
        }
        .to_string();

        rule_maps_by_class_name
          .entry(class_name)
          .and_modify(|rule_map| {
            rule_map
              .entry(rule_set_node.id())
              .or_insert(rule_set.to_owned());
          })
          .or_insert(HashMap::from([(rule_set_node.id(), rule_set)]));
      }
    }

    // Convert intermediate completions into final list.
    let mut completions: Vec<(String, String)> = vec![];

    // Reserve space for completions to prevent reallocations.
    completions.reserve(rule_maps_by_class_name.len());

    // I tried a .reserve and .drain approach, and there was no benchmarking
    // improvement. So I opted for the more readable version.
    for class_name in rule_maps_by_class_name.keys().sorted() {
      let rule_map = rule_maps_by_class_name.get(class_name).unwrap().to_owned();
      let rule_sets: Vec<String> = rule_map.into_values().sorted().collect();

      completions.push((class_name.clone(), rule_sets.join("\n\n")));
    }

    self.completions = completions;
    &self.completions
  }

  fn build_parser(&self) -> Result<Parser, &'static str> {
    let mut parser = Parser::new();
    if parser.set_language(tree_sitter_css::language()).is_err() {
      error!("Error loading scss grammar");
      return Err("Error loading scss grammar");
    }
    Ok(parser)
  }
}

impl Default for AutocompletionEngine {
  fn default() -> Self {
    Self::new()
  }
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
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type=\"password\"].focus,\ninput[type=\"password\"]:focus,\ninput[type=\"email\"].focus,\ninput[type=\"email\"]:focus,\ninput[type=\"number\"].focus,\ninput[type=\"number\"]:focus,\ninput[type=\"tel\"].focus,\ninput[type=\"tel\"]:focus,\ninput[type=\"url\"].focus,\ninput[type=\"url\"]:focus,\ninput[type=\"search\"].focus,\ninput[type=\"search\"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "focus",
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type=\"password\"].focus,\ninput[type=\"password\"]:focus,\ninput[type=\"email\"].focus,\ninput[type=\"email\"]:focus,\ninput[type=\"number\"].focus,\ninput[type=\"number\"]:focus,\ninput[type=\"tel\"].focus,\ninput[type=\"tel\"]:focus,\ninput[type=\"url\"].focus,\ninput[type=\"url\"]:focus,\ninput[type=\"search\"].focus,\ninput[type=\"search\"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "focused",
        "input[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type=\"password\"].focus,\ninput[type=\"password\"]:focus,\ninput[type=\"email\"].focus,\ninput[type=\"email\"]:focus,\ninput[type=\"number\"].focus,\ninput[type=\"number\"]:focus,\ninput[type=\"tel\"].focus,\ninput[type=\"tel\"]:focus,\ninput[type=\"url\"].focus,\ninput[type=\"url\"]:focus,\ninput[type=\"search\"].focus,\ninput[type=\"search\"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}"
      ),
      (
        "input-contrast",
        "input.input-contrast,\n.input-contrast {\n  background-color: #fafafa;\n}\n\ninput.input-contrast:focus,\n.input-contrast:focus {\n  background-color: #fff;\n}"
      ),
      (
        "search-page-label",
        "input[type=\"text\"],\n#adv_code_search .search-page-label,\ninput[type=\"password\"],\ninput[type=\"email\"],\ninput[type=\"number\"],\ninput[type=\"tel\"],\ninput[type=\"url\"],\ninput[type=\"search\"],\ntextarea {\n  min-height: 34px;\n  padding: 7px 8px;\n  font-size: 13px;\n  color: #333;\n  vertical-align: middle;\n  background-color: #fff;\n  background-repeat: no-repeat;\n  background-position: right center;\n  border: 1px solid #ccc;\n  border-radius: 3px;\n  outline: none;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075);\n}\n\ninput[type=\"text\"].focus,\n#adv_code_search .focus.search-page-label,\ninput[type=\"text\"]:focus,\n.focused .drag-and-drop,\n#adv_code_search .search-page-label:focus,\ninput[type=\"password\"].focus,\ninput[type=\"password\"]:focus,\ninput[type=\"email\"].focus,\ninput[type=\"email\"]:focus,\ninput[type=\"number\"].focus,\ninput[type=\"number\"]:focus,\ninput[type=\"tel\"].focus,\ninput[type=\"tel\"]:focus,\ninput[type=\"url\"].focus,\ninput[type=\"url\"]:focus,\ninput[type=\"search\"].focus,\ninput[type=\"search\"]:focus,\ntextarea.focus,\ntextarea:focus {\n  border-color: #51a7e8;\n  box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.075),\n    0 0 5px rgba(81, 167, 232, 0.5);\n}",
      ),
      (
        "wrapper",
        "#peek .wrapper {\n  width: 860px !important;\n  padding: 0;\n}\n\n#peek2 .wrapper {\n  border: 1px solid red;\n}",
      ),
    ]
  }

  #[test]
  fn test_parser_failure() {
    let mut engine = AutocompletionEngine::new();
    let completions =
      engine.get_all_completions_for_files(vec!["invalid_file_path.css".to_string()]);
    assert!(completions.is_empty());
  }

  #[test]
  fn can_get_completions() {
    let mut engine = AutocompletionEngine::new();

    let actual =
      engine.get_all_completions_for_files(vec!["./__test__/test.atom.io.css".to_string()]);
    let expected = get_list();

    for (i, (class_name, rule_set)) in expected.iter().enumerate() {
      dbg!(class_name);
      assert_eq!(rule_set.to_string(), actual[i].1);
    }

    assert_eq!(actual.len(), expected.len());
  }

  // This is cheating a bit, but I don't have time or energy to inline a long string right now.
  #[test]
  fn can_get_completions_as_string() {
    let mut engine = AutocompletionEngine::new();

    let list = get_list();
    let actual =
      engine.get_all_completions_as_string(vec!["./__test__/test.atom.io.css".to_string()]);
    let expected = serde_json::to_string(&list).expect("Could not convert class hashmap to string");
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
  fn handles_empty_file_gracefully() {
    let mut engine = AutocompletionEngine::new();
    let completions =
      engine.get_all_completions_for_files(vec!["./__test__/empty.css".to_string()]);
    assert!(completions.is_empty());
  }

  #[test]
  fn can_invalidate_cache() {
    let mut engine = AutocompletionEngine::new();
    engine.get_all_completions_for_files(vec!["./__test__/test.atom.io.css".to_string()]);
    assert!(!engine.completions.is_empty());

    engine.invalidate_cache();
    assert!(engine.completions.is_empty());
  }

  #[test]
  fn uses_cache_for_subsequent_calls() {
    let mut engine = AutocompletionEngine::new();
    let first_call = engine
      .get_all_completions_for_files(vec!["./__test__/test.atom.io.css".to_string()])
      .clone();
    let second_call =
      engine.get_all_completions_for_files(vec!["./__test__/test.atom.io.css".to_string()]);

    assert_eq!(first_call.len(), second_call.len());

    // The lengths are asserted to be equal, so we can just iterate over
    // second_call for the comparison.
    for (i, (class_name, rule_set)) in first_call.iter().enumerate() {
      assert_eq!(class_name, &second_call[i].0);
      assert_eq!(rule_set, &second_call[i].1);
    }
  }
}
