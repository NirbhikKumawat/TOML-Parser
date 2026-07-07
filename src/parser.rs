use crate::config_error::ConfigError;
use crate::token::{SpannedToken, TokenKind};
use crate::toml_value::TomlValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn current(&self) -> Option<&SpannedToken> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
    fn skip_newlines(&mut self) {
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::NewLine | TokenKind::Comment(_) => {
                    self.advance();
                }
                _ => break,
            }
        }
    }
    fn parse_value(&mut self) -> Result<TomlValue, ConfigError> {
        let token = self.current().cloned();
        match token {
            Some(t) => match &t.kind {
                TokenKind::StringLit(s) => {
                    let value = TomlValue::String(s.clone());
                    self.advance();
                    Ok(value)
                }
                TokenKind::Integer(n) => {
                    let value = TomlValue::Integer(*n);
                    self.advance();
                    Ok(value)
                }
                TokenKind::Float(n) => {
                    let value = TomlValue::Float(*n);
                    self.advance();
                    Ok(value)
                }
                TokenKind::Boolean(b) => {
                    let value = TomlValue::Boolean(*b);
                    self.advance();
                    Ok(value)
                }
                TokenKind::LBracket => {
                    self.advance();
                    let mut elements: Vec<TomlValue> = Vec::new();
                    let mut expect_comma = false;
                    while let Some(token) = self.current().cloned() {
                        self.skip_newlines();
                        if expect_comma {
                            match token.kind {
                                TokenKind::Comma => {
                                    expect_comma = false;
                                    self.advance();
                                }
                                TokenKind::RBracket => {
                                    self.advance();
                                    return Ok(TomlValue::Array(elements));
                                }
                                _ => {
                                    return Err(ConfigError::ExpectedToken {
                                        line: token.line,
                                        col: token.col,
                                        expected: ",".to_string(),
                                        found: format!("{:?}", t.kind),
                                    });
                                }
                            }
                        } else {
                            match token.kind {
                                TokenKind::RBracket => {
                                    self.advance();
                                    return Ok(TomlValue::Array(elements));
                                }
                                _ => {
                                    match self.parse_value() {
                                        Ok(value) => {
                                            elements.push(value);
                                        }
                                        Err(err) => {
                                            return Err(err);
                                        }
                                    };
                                }
                            }
                            expect_comma = true;
                        }
                    }
                    Err(ConfigError::UnexpectedCharacter {
                        line: t.line,
                        col: t.col,
                        expected: "]".to_string(),
                        found: "end of input".to_string(),
                    })
                }
                _ => Err(ConfigError::ExpectedToken {
                    line: t.line,
                    col: t.col,
                    expected: "value (string, integer, float, or boolean)".to_string(),
                    found: format!("{:?}", t.kind),
                }),
            },
            None => Err(ConfigError::ExpectedToken {
                line: 0,
                col: 0,
                expected: "value".to_string(),
                found: "end of input".to_string(),
            }),
        }
    }
    pub fn parse(&mut self) -> Result<TomlValue, ConfigError> {
        let mut pairs: HashMap<String, TomlValue> = HashMap::new();
        let mut current_path: Vec<String> = Vec::new();
        while let Some(token) = self.current().cloned() {
            match &token.kind {
                TokenKind::Comment(_) | TokenKind::NewLine => {
                    self.advance();
                }
                TokenKind::LBracket => {
                    current_path = self.parse_table_header()?;
                }
                TokenKind::DoubleLBracket => {
                    current_path = self.parse_array_table_header()?;
                    self.push_new_array_table(&current_path, &mut pairs)?;
                }
                TokenKind::Identifier(key) | TokenKind::StringLit(key) => {
                    self.advance();
                    self.parse_key_value(&current_path, key, &mut pairs)?;
                }
                _ => {
                    return Err(ConfigError::UnexpectedCharacter {
                        line: token.line,
                        col: token.col,
                        expected: "key, table header, or comment".to_string(),
                        found: format!("{:?}", token.kind),
                    });
                }
            }
        }
        Ok(TomlValue::Table(pairs))
    }
    fn parse_array_table_header(&mut self) -> Result<Vec<String>, ConfigError> {
        self.advance();
        let mut expect_dot = false;
        let mut dotted: Vec<String> = Vec::new();
        let mut line = 0;
        let mut col = 0;
        while let Some(token) = self.current().cloned() {
            if expect_dot {
                match &token.kind {
                    TokenKind::DoubleRBracket => {
                        self.advance();
                        return Ok(dotted);
                    }
                    TokenKind::Dot => {
                        expect_dot = false;
                        self.advance();
                    }
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: token.line,
                            col: token.col,
                            expected: "'.' or ']]'".to_string(),
                            found: format!("{:?}", token.kind),
                        });
                    }
                }
            } else {
                match &token.kind {
                    TokenKind::Identifier(t) => {
                        dotted.push(t.clone());
                        expect_dot = true;
                        self.advance();
                    }
                    TokenKind::StringLit(t) => {
                        dotted.push(t.clone());
                        expect_dot = true;
                        self.advance();
                    }
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: token.line,
                            col: token.col,
                            expected: "string or identifier".to_string(),
                            found: format!("{:?}", token.kind),
                        });
                    }
                }
            }
            line = token.line;
            col = token.col;
        }
        Err(ConfigError::ExpectedToken {
            line,
            col,
            expected: "']]'".to_string(),
            found: "end of the file".to_string(),
        })
    }
    fn parse_table_header(&mut self) -> Result<Vec<String>, ConfigError> {
        self.advance();
        let mut expect_dot = false;
        let mut dotted: Vec<String> = Vec::new();
        let mut line = 0;
        let mut col = 0;
        while let Some(token) = self.current().cloned() {
            if expect_dot {
                match &token.kind {
                    TokenKind::RBracket => {
                        self.advance();
                        return Ok(dotted);
                    }
                    TokenKind::Dot => {
                        expect_dot = false;
                        self.advance();
                    }
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: token.line,
                            col: token.col,
                            expected: "'.' or ']'".to_string(),
                            found: format!("{:?}", token.kind),
                        });
                    }
                }
            } else {
                match &token.kind {
                    TokenKind::Identifier(t) => {
                        dotted.push(t.clone());
                        expect_dot = true;
                        self.advance();
                    }
                    TokenKind::StringLit(t) => {
                        dotted.push(t.clone());
                        expect_dot = true;
                        self.advance();
                    }
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: token.line,
                            col: token.col,
                            expected: "string or identifier".to_string(),
                            found: format!("{:?}", token.kind),
                        });
                    }
                }
            }
            line = token.line;
            col = token.col;
        }
        Err(ConfigError::ExpectedToken {
            line,
            col,
            expected: "']'".to_string(),
            found: "end of the file".to_string(),
        })
    }
    fn parse_key_value(
        &mut self,
        path: &Vec<String>,
        key: &str,
        root: &mut HashMap<String, TomlValue>,
    ) -> Result<(), ConfigError> {
        let mut current_root = root;
        for header in path {
            let next_node = current_root
                .entry(header.clone())
                .or_insert_with(|| TomlValue::Table(HashMap::new()));

            match next_node {
                TomlValue::Table(inner) => {
                    current_root = inner;
                }
                TomlValue::Array(arr) => {
                    if let Some(TomlValue::Table(last_table)) = arr.last_mut() {
                        current_root = last_table;
                    } else {
                        return Err(ConfigError::UnexpectedCharacter {
                            line: 0,
                            col: 0,
                            expected: "an array of tables".to_string(),
                            found: "an empty or non-table array".to_string(),
                        });
                    }
                }
                _ => {
                    return Err(ConfigError::UnexpectedCharacter {
                        line: 0,
                        col: 0,
                        expected: "a table".to_string(),
                        found: "a conflicting value".to_string(),
                    });
                }
            }
        }
        let eq_token = self.current();
        match eq_token {
            Some(t) if t.kind == TokenKind::Equal => {
                self.advance();
            }
            Some(t) => {
                return Err(ConfigError::ExpectedToken {
                    line: t.line,
                    col: t.col,
                    expected: "=".to_string(),
                    found: format!("{:?}", t.kind),
                });
            }
            None => {
                return Err(ConfigError::MissingValue {
                    line: 0,
                    col: 1,
                    key: key.parse().unwrap(),
                });
            }
        }
        let value = self.parse_value()?;
        current_root.insert(key.parse().unwrap(), value);
        Ok(())
    }
    fn push_new_array_table(
        &mut self,
        path: &[String],
        root: &mut HashMap<String, TomlValue>,
    ) -> Result<(), ConfigError> {
        if path.is_empty() {
            return Ok(());
        }
        let mut current_root = root;

        for header in path.iter().take(path.len().saturating_sub(1)) {
            let next_node = current_root
                .entry(header.clone())
                .or_insert_with(|| TomlValue::Table(HashMap::new()));

            match next_node {
                TomlValue::Table(inner) => {
                    current_root = inner;
                }
                TomlValue::Array(arr) => {
                    if let Some(TomlValue::Table(last_table)) = arr.last_mut() {
                        current_root = last_table;
                    } else {
                        return Err(ConfigError::UnexpectedCharacter {
                            line: 0,
                            col: 0,
                            expected: "valid array table".to_string(),
                            found: "invalid array table".to_string(),
                        });
                    }
                }
                _ => {
                    return Err(ConfigError::UnexpectedCharacter {
                        line: 0,
                        col: 0,
                        expected: "valid array table".to_string(),
                        found: "invalid array table".to_string(),
                    });
                }
            }
        }
        let last_key = path.last().unwrap();
        let target = current_root
            .entry(last_key.clone())
            .or_insert_with(|| TomlValue::Array(Vec::new()));
        match target {
            TomlValue::Array(arr) => {
                arr.push(TomlValue::Table(HashMap::new()));
                Ok(())
            }
            _ => Err(ConfigError::UnexpectedCharacter {
                line: 0,
                col: 0,
                expected: "an array".to_string(),
                found: "a statically defined table or value".to_string(),
            }),
        }
    }
}
