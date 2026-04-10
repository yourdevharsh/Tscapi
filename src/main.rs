
// mod cli;
mod tsparser;


fn main() {
    // cli::return_args();

    let mut parser = tsparser::TSParser::new();

    let file_path = "C:/NodeJS/System/Tscapi/try.ts";

    let Ok(code) = std::fs::read_to_string(file_path) else {
        eprintln!("Failed to read file.");
        return;
    };

    let Ok(tree) = parser.parse(&code) else {
        eprintln!("Failed to parse code!");
        return;
    };

    let root_node = tree.root_node();

    // let kind = root_node.kind();
    // let text = root_node.utf8_text(code.as_bytes()).unwrap_or("");
    
    for i in 0..root_node.named_child_count() {
        let child = root_node.child(i as u32).unwrap();
        print!("<{}> {} \n", child, child.utf8_text(code.as_bytes()).unwrap_or(""));
    }

    let mut cursor = root_node.walk();

    fn traverse(cursor: &mut tree_sitter::TreeCursor, code: &str) {
        loop {
            let node = cursor.node();

            println!("Kind: {}", node.kind());

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

    // println!("{:?}, {:?}", kind, text);

    // match parser.parse(code) {
    //     Ok(tree) => println!("Successfully parsed: {:?}", tree.root_node().child(0)),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
}
