use std::fs;
use toml::config_error::{ConfigError, format_error};
use toml::lexer::Lexer;
use toml::parser::Parser;
use toml::schema::{parse_schema, validate};
use toml::toml_value::{TomlValue, display};

fn run(config_path: &str, schema_path: &str) -> Result<(), Vec<ConfigError>> {
    let config_source = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(e) => return Err(vec![ConfigError::from(e)]),
    };
    let schema_source = match fs::read_to_string(schema_path) {
        Ok(s) => s,
        Err(e) => return Err(vec![ConfigError::from(e)]),
    };

    let mut config_lexer = Lexer::new(&config_source);
    let config_tokens = match config_lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return Err(vec![e]),
    };
    let mut config_parser = Parser::new(config_tokens);
    let config_value = match config_parser.parse() {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };

    let config_map = match &config_value {
        TomlValue::Table(table) => table,
        _ => return Err(vec![]),
    };

    let mut schema_lexer = Lexer::new(&schema_source);
    let schema_tokens = match schema_lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => return Err(vec![e]),
    };
    let mut schema_parser = Parser::new(schema_tokens);
    let schema_value = match schema_parser.parse() {
        Ok(v) => v,
        Err(e) => return Err(vec![e]),
    };

    let schema = match parse_schema(&schema_value) {
        Ok(s) => s,
        Err(e) => return Err(vec![e]),
    };

    validate(&schema, &config_map).map_err(|e| vec![e])?;
    println!("Config is valid!");

    println!("Parsed Configuration:\n{:#?}", config_value);

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (config_path, schema_path) = if args.len() >= 3 {
        (args[1].clone(), args[2].clone())
    } else {
        (
            String::from("examples/valid_config.toml"),
            String::from("examples/schema.toml"),
        )
    };

    match run(&config_path, &schema_path) {
        Ok(()) => {}
        Err(errors) => {
            let source = match fs::read_to_string(&config_path) {
                Ok(s) => s,
                Err(_) => String::new(),
            };
            for err in &errors {
                eprintln!("{}", format_error(err, &source));
                eprintln!();
            }
            std::process::exit(1);
        }
    }
}
