mod utils;

use std::collections::HashMap;

use tree_sitter::{Parser, Query, QueryCursor};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

type Completions = HashMap<String, Vec<String>>;

#[wasm_bindgen]
pub struct AutocompletionEngine {
    // Would love to type this as a proper hashmap, but the wasm abi doesn't support that.
    completions: String,
}

/// Public API exposed via WASM.
#[wasm_bindgen]
impl AutocompletionEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(&self) -> Self {
        set_panic_hook();

        let completions = self.get_completions();
        dbg!(completions);
        // Self { completions }
        Self {
            completions: String::from(""),
        }
    }

    #[allow(unused_variables)]
    #[wasm_bindgen(js_name = doStuff)]
    pub fn do_stuff(&self) {
        log("doing stuff!");
        log(&serde_json::to_string(&self.completions).expect("failed to serialize"));
    }

    fn get_completions(&self) -> String {
        let code = include_str!("./atom.io.css");
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_css::language())
            .expect("Error loading scss grammar");
        let tree = parser.parse(code, None).expect("Could not parse code");

        let query = Query::new(
            tree_sitter_css::language(),
            r#"(class_selector
        ) @class-name"#,
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

        // dbg!(classes);
        log(&serde_json::to_string(&classes).expect("Could not convert class hashmap to string"));
        serde_json::to_string(&classes).expect("Could not convert class hashmap to string")
    }
}
