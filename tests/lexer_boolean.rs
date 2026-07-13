use insta::assert_debug_snapshot;
use toml::lexer::Lexer;

#[test]
fn lexer_boolean() {
    let toml = "boolean =  true";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
