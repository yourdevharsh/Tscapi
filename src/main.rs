// mod cli;
mod tsparser;
mod freader;

fn main() {
    // cli::return_args();

    let mut parser = tsparser::TSParser::new();

    let file_path = "C:/NodeJS/System/Tscapi/try.ts";

    let Ok(code) = freader::read_file(file_path) else {
        eprintln!("Failed to read file!");
        return;
    };

    let Ok(tree) = parser.parse(&code) else {
        eprintln!("Failed to parse code!");
        return;
    };

    let root_node = tree.root_node();

    let mut cursor = root_node.walk();

    let mut ast: std::collections::HashMap<String, Box<dyn std::any::Any>> = std::collections::HashMap::new();

    fn traverse(cursor: &mut tree_sitter::TreeCursor, code: &str) {
        loop {
            let node = cursor.node();

            if node.is_named() {
                // println!(
                //     "Kind: {} \n Text: {} \n Field Name: {:?} \n",
                //     node.kind(),
                //     node.utf8_text(code.as_bytes()).unwrap_or(""),
                //     cursor.field_name()
                // );


            }

            if cursor.goto_first_child() {
                traverse(cursor, code);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    traverse(&mut cursor, &code);
}
