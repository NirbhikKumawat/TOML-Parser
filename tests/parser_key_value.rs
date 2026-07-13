use insta::assert_debug_snapshot;
use toml::lexer::Lexer;
use toml::parser::Parser;

#[test]
fn parser_key_value_nested_braces() {
    let toml = include_str!("../examples/lexer.toml");
    let mut lexer = Lexer::new(toml);
    let tokenized = lexer.tokenize();
    let tokens = match tokenized {
        Ok(tokens) => tokens,
        Err(e) => panic!("{:?}", e),
    };
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse();
    match parsed {
        Ok(parsed) => {
            let braces = parsed
                .get("root")
                .unwrap()
                .get("braces")
                .unwrap()
                .get("key");
            assert_debug_snapshot!(braces);
            let deep_nested = parsed
                .get("root")
                .unwrap()
                .get("numbers")
                .unwrap()
                .get("floats")
                .unwrap()
                .get("normal")
                .unwrap()
                .get("float1")
                .unwrap();
            assert_debug_snapshot!(deep_nested);
            let array_val = parsed
                .get("root")
                .unwrap()
                .get("numbers")
                .unwrap()
                .get("floats")
                .unwrap()
                .get("special")
                .unwrap()
                .get_at_index("float7", 0)
                .unwrap();
            assert_debug_snapshot!(array_val);
            let array_table = parsed
                .get_at_index("ArrayOfTables", 1)
                .unwrap()
                .get("index")
                .unwrap();
            assert_debug_snapshot!(array_table);
        }
        Err(e) => panic!("{:?}", e),
    }
}
