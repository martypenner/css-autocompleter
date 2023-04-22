use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    let code = include_str!("./atom.io.css");
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_css::language())
        .expect("Error loading scss grammar");
    let tree = parser.parse(code, None).unwrap();

    let query = Query::new(tree_sitter_css::language(), "(class_selector) @class-name").unwrap();
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), code.as_bytes());
    for each_match in matches {
        for capture in each_match.captures {
            dbg!(capture.node.utf8_text(code.as_bytes()).unwrap());
        }
    }
}
