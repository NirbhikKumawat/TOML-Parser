use insta::assert_debug_snapshot;
use toml::lexer::Lexer;

#[test]
fn lexer_integer() {
    let toml = "integer = -100";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
#[test]
fn lexer_binary_integer() {
    let toml = "intb = 0b100101010101";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_hex_integer() {
    let toml = "inth = 0x34b34cf";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_octal_integer() {
    let toml = "into = 0o235647235";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_underscore_integer() {
    let toml = "integer = -1_0_0";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_binary_underscore(){
    let toml = "intb = 0b100_1010_10_101";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_hex_underscore(){
    let toml = "inth = 0x34_b34_cf";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_octal_underscore(){
    let toml = "into = 0o2_35_64723_5";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}