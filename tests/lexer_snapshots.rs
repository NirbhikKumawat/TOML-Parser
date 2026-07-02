use insta::assert_debug_snapshot;
use toml::lexer::Lexer;

#[test]
fn test_lexer() {
    let toml = include_str!("../examples/lexer.toml");
    let mut lexer = Lexer::new(toml);
    let tokenized = lexer.tokenize();
    assert_debug_snapshot!(tokenized);
}
