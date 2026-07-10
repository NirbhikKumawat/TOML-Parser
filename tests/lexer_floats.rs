use insta::assert_debug_snapshot;
use toml::lexer::Lexer;

#[test]
fn test_float() {
    let toml = "float = 3.14";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
#[test]
fn test_float_underscores() {
    let toml = "float = 3.14_15";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
#[test]
fn test_float_underscore_start(){
    let toml = "float = _3.1415";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_underscore_end(){
    let toml = "float = 3.1415_";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_underscore_before_dot(){
    let toml = "float = 3_.1415";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_underscore_after_dot(){
    let toml = "float = 3._1415";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_special(){
    let toml = "float = nan";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_special_sign(){
    let toml = "float = -inf";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_scientific() {
    let toml = "float = 6.6e23";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
#[test]
fn test_float_scientific_exponent_sign(){
    let toml = "float = 6.6e+23";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_scientific_exponent_underscore(){
    let toml = "float = 6.6e+2_3";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_scientific_no_dot() {
    let toml = "float = 1e6";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_float_scientific_dot_after_exponent() {
    let toml = "float = 1e2.5";
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}