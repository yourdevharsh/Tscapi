use std::process::id;

// mod cli;
mod tsparser;
mod freader;
mod data_formats;

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

    let mut ast: data_formats::IR = data_formats::IR::new();

    fn traverse(
        cursor: &mut tree_sitter::TreeCursor,
        code: &str,
        ast: &mut data_formats::IR
    ) {
        loop {
            let node = cursor.node();

            if node.is_named() {
                // println!(
                //     "Kind: {} \n Text: {} \n Field Name: {:?} \n",
                //     node.kind(),
                //     node.utf8_text(code.as_bytes()).unwrap_or(""),
                //     cursor.field_name()
                // );

                if node.kind() == "function_declaration" {
                    let Some(name_node) = node.child_by_field_name("name") else {
                        eprint!("No function name node");
                        return;
                    };

                    let function_name = name_node
                        .utf8_text(code.as_bytes())
                        .unwrap_or("")
                        .to_string();

                    let function_text = node.utf8_text(code.as_bytes()).unwrap_or("").to_string();

                    ast.add_node_type(node.kind().to_string());
                }
            }

            if cursor.goto_first_child() {
                traverse(cursor, code, ast);
                cursor.goto_parent();
            }

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    traverse(&mut cursor, &code, &mut ast);

    // println!("{:?}", ast.id);
}
