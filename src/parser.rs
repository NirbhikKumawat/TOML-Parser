use crate::ConfigError;
use crate::token::{SpannedToken, TokenKind};
use crate::toml_value::TomlValue;

#[derive(Debug, Clone)]
struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<SpannedToken>) -> Self {
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
        let token = self.current();
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
    fn parse(&mut self) -> Result<TomlValue, ConfigError> {
        let mut pairs: Vec<(String, TomlValue)> = Vec::new();

        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Comment(_) | TokenKind::NewLine => {
                    self.advance();
                }
                TokenKind::TableHeader(section) => {
                    let section_name = section.clone();
                    self.advance();
                    self.skip_newlines();

                    let mut section_pairs: Vec<(String, TomlValue)> = Vec::new();

                    while let Some(t) = self.current() {
                        match &t.kind {
                            TokenKind::Comment(_) | TokenKind::NewLine => {
                                self.advance();
                            }
                            TokenKind::TableHeader(_) => {
                                break;
                            }
                            TokenKind::Identifier(key) => {
                                let key_str = key.clone();
                                let key_line = t.line;
                                self.advance();

                                let eq_token = self.current();
                                match eq_token {
                                    Some(eq) if eq.kind == TokenKind::Equal => {
                                        self.advance();
                                    }
                                    Some(eq) => {
                                        return Err(ConfigError::ExpectedToken {
                                            line: eq.line,
                                            col: eq.col,
                                            expected: "=".to_string(),
                                            found: format!("{:?}", eq.kind),
                                        });
                                    }
                                    None => {
                                        return Err(ConfigError::MissingValue {
                                            line: key_line,
                                            col: 1,
                                            key: key_str,
                                        });
                                    }
                                }

                                let value = self.parse_value()?;
                                section_pairs.push((key_str, value));
                                self.skip_newlines();
                            }
                            _ => {
                                return Err(ConfigError::UnexpectedCharacter {
                                    line: t.line,
                                    col: t.col,
                                    expected: "key or table header".to_string(),
                                    found: format!("{:?}", t.kind),
                                });
                            }
                        }
                    }
                    pairs.push((section_name, TomlValue::Table(section_pairs)));
                }
                TokenKind::Identifier(key) => {
                    let key_str = key.clone();
                    let key_line = token.line;
                    self.advance();

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
                                line: key_line,
                                col: 1,
                                key: key_str,
                            });
                        }
                    }
                    let value = self.parse_value()?;
                    pairs.push((key_str, value));
                    self.skip_newlines();
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
}
