#[macro_use]
extern crate napi_derive;

use std::collections::HashMap;

use tree_sitter::{Parser, Query, QueryCursor};

type Completions = HashMap<String, Vec<String>>;

#[napi]
pub fn get_completions_as_string() -> String {
  let classes = get_completions();
  serde_json::to_string(&classes).expect("Could not convert class hashmap to string")
}

fn get_completions() -> Completions {
  let code = include_str!("./atom.io.css");
  let mut parser = Parser::new();
  parser
    .set_language(tree_sitter_css::language())
    .expect("Error loading scss grammar");
  let tree = parser.parse(code, None).expect("Could not parse code");

  let query = Query::new(
    tree_sitter_css::language(),
    r#"(class_selector (class_name) @class-name)"#,
  )
  .expect("Could not create tree sitter query");
  let mut query_cursor = QueryCursor::new();
  let matches = query_cursor.matches(&query, tree.root_node(), code.as_bytes());
  let mut classes: Completions = HashMap::new();

  for each_match in matches {
    for capture in each_match.captures {
      let class_name = capture
        .node
        .utf8_text(code.as_bytes())
        .expect("Could not convert node to utf8 text")
        .to_string();
      // let _existing = classes.get(class_name);
      // classes.insert(class_name, existing.unwrap_or_default());
      classes.insert(class_name, vec![]);
    }
  }

  dbg!(&classes);
  classes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut classes: Completions = HashMap::new();
    classes.insert(String::from("wrapper"), vec![]);
    classes.insert(String::from("focus"), vec![]);
    classes.insert(String::from("focused"), vec![]);
    classes.insert(String::from("input-contrast"), vec![]);
    classes.insert(String::from("search-page-label"), vec![]);
    classes.insert(String::from("drag-and-drop"), vec![]);

    assert_eq!(get_completions(), classes);
  }
}
