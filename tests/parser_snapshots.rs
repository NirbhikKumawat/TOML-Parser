use insta::{assert_debug_snapshot, assert_json_snapshot};
use toml::lexer::Lexer;
use toml::parser::Parser;

#[test]
fn test_parser() {
    let toml = include_str!("../examples/lexer.toml");
    let mut lexer = Lexer::new(toml);
    let tokenized = lexer.tokenize();
    //assert_debug_snapshot!(tokenized);
    let tokens = match tokenized {
        Ok(tokens) => tokens,
        Err(e) => panic!("{:?}", e),
    };
    let mut parser = Parser::new(tokens);
    //assert_debug_snapshot!(parser);
    let parsed = parser.parse();
    match parsed {
        Ok(parsed) => {
            assert_json_snapshot!(parsed);
        }
        Err(e) => panic!("{:?}", e),
    }
}
