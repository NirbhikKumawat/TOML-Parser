use insta::assert_debug_snapshot;
use toml::lexer::Lexer;
use toml::parser::Parser;

#[test]
fn parser_key_value() {
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
            let val = parsed
                .get("root")
                .unwrap()
                .get("numbers")
                .unwrap()
                .get("floats")
                .unwrap();
            assert_debug_snapshot!(val);
        }
        Err(e) => panic!("{:?}", e),
    }
}
