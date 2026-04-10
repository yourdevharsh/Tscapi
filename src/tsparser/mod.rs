use tree_sitter::{ Parser };

pub struct TSParser {
    parser: Parser,
}

impl TSParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("Error loading grammar");

        Self { parser }
    }

    pub fn parse(&mut self, code: &str) -> Result<tree_sitter::Tree, String> {
        let tree = match self.parser.parse(code, None) {
            Some(v) => v,
            None => {
                return Err("Parse failed".into());
            }
        };
        Ok(tree)
    }
}
