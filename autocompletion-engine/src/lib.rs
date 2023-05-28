#[macro_use]
extern crate napi_derive;

use std::{
  collections::{HashMap, HashSet},
  fs::read_to_string,
  println,
};

use itertools::Itertools;
use tree_sitter::{Parser, Query, QueryCursor};

type Completions = Vec<(ClassName, RuleSet)>;
type IntermediateCompletions = HashMap<ClassName, RuleSetMap>;
type ClassName = String;
type RuleSet = String;

type RuleSetMap = HashMap<RuleSetId, HelpDoc>;
type RuleSetId = usize;
type HelpDoc = String;

#[napi]
pub struct AutocompletionEngine {
  completions: Completions,
}

#[napi]
impl AutocompletionEngine {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      completions: vec![],
    }
  }

  #[napi]
  pub fn get_all_completions_as_string(&mut self, files: Vec<String>) -> String {
    let completions = self.get_all_completions_for_files(files);
    serde_json::to_string(completions).expect("Could not convert class hashmap to string")
  }

  #[napi]
  pub fn invalidate_cache(&mut self) {
    self.completions = vec![];
  }

  // TODO: split this thing up for a bit more readability
  // TODO: log errors instead of panicking
  // TODO: we're currently structured around classnames. This is nice for looping and
  // creating the final completions list, but makes it harder to invalidate the cache
  // for a single file. Need to rethink this.
  fn get_all_completions_for_files(&mut self, files: Vec<String>) -> &Completions {
    if self.completions.len() > 0 {
      return &self.completions;
    }

    // Ensure we don't operate on the same file twice.
    let files: HashSet<String> = files.into_iter().collect();

    let mut parser = Parser::new();
    parser
      .set_language(tree_sitter_css::language())
      .expect("Error loading scss grammar");
    let query = get_class_selectors_query_for_tree();
    let mut query_cursor = QueryCursor::new();

    let mut rule_maps_by_class_name: IntermediateCompletions = HashMap::new();

    for path in files {
      let code = read_to_string(&path).expect("Could not read file");
      let tree = parser.parse(&code, None).expect("Could not parse code");
      let code = code.as_bytes();

      let matches = query_cursor.matches(&query, tree.root_node(), code);

      for each_match in matches {
        let [class_selector, class_name] = each_match.captures else {
      println!("Could not destructure captures");
      continue;
    };

        let class_selector = class_selector.node;
        let class_name = class_name.node;

        // Find parent rule set.
        let mut parent = class_selector.parent();
        loop {
          match parent {
            Some(found_parent) => {
              if found_parent.kind() == "rule_set" {
                break;
              } else {
                parent = found_parent.parent();
              }
            }
            None => break,
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
                .to_string()
            );
            continue;
          }
        };

        let rule_set = rule_set_node
          .utf8_text(code)
          .expect("Could not convert node to utf8 text")
          .to_string();

        let class_name = class_name
          .utf8_text(code)
          .expect("Could not convert node to utf8 text")
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
    // TODO: there HAS to be a better way to convert a map to a final string, but
    // `collect` was pretty cumbersome, and I gave up.
    let mut completions: Vec<(String, String)> = vec![];
    for class_name in rule_maps_by_class_name.keys().sorted() {
      let rule_map = rule_maps_by_class_name.get(class_name).unwrap().to_owned();
      let rule_sets: Vec<String> = rule_map.into_values().sorted().collect();

      completions.push((class_name.clone(), rule_sets.join("\n\n")));
    }

    self.completions = completions;
    &self.completions
  }
}

fn get_class_selectors_query_for_tree() -> Query {
  let query = Query::new(
    tree_sitter_css::language(),
    r#"
      (class_selector
        (class_name) @class_name
      ) @class_selector
    "#,
  )
  .expect("Could not create tree sitter query");

  query
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
  fn operates_only_once_on_each_file() {
    // TODO: not sure how to test this
  }
}
