use insta::assert_debug_snapshot;
use toml::lexer::Lexer;

#[test]
fn lexer_strings() {
    let toml = r#"string1 = "single line string \t single line string""#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_literal() {
    let toml = r#"string1 = 'single line string \t single line string'"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_strings_multiline() {
    let toml = r#"string2 = """
This is a multinine
Really a multine string

Nice string
""""#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_literal_multiline() {
    let toml = r#"string1 = '''
This is a multiline literal \n\t\r
"Good literal it is" \"   ><

Nice string
'''"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_unterminated_string() {
    let toml = r#"string1 = "single line string \t single line string"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_unterminated_multiline_string() {
    let toml = r#"string2 = """
This is a multinine
Really a multine string

Nice string
"""#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_unterminated_string_literal() {
    let toml = r#"string1 = 'single line string \t single line string"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_unterminated_string_literal_multiline() {
    let toml = r#"string1 = '''
This is a multiline literal \n\t\r
"Good literal it is" \"   ><

Nice string
''"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}

#[test]
fn lexer_middle_multiline_literal(){
    let toml = r#"string1 = '''
This is a mult'ili'ne literal \n\t\r
"Good literal it is" \"   ><

Nice string
'''"#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}
#[test]
fn lexer_middle_string_multiline() {
    let toml = r#"string2 = """
This i"s a mul"tinine
Really" a multine string

Nice string
""""#;
    let mut lexer = Lexer::new(toml);
    let tokens = lexer.tokenize();
    assert_debug_snapshot!(tokens);
}