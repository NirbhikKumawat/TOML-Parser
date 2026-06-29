use crate::ConfigError;
use crate::token::{SpannedToken, TokenKind};

#[derive(Debug)]
struct Lexer {
    input: Vec<u8>,
    pos: usize,
    line: usize,
    col: usize,
}
impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.as_bytes().to_vec(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    fn current(&self) -> Option<u8> {
        if self.pos < self.input.len() {
            Some(self.input[self.pos])
        } else {
            None
        }
    }
    fn advance(&mut self) {
        if self.pos < self.input.len() {
            if self.input[self.pos] == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.pos += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        if self.pos + 1 < self.input.len() {
            Some(self.input[self.pos + 1])
        } else {
            None
        }
    }
    fn skip_whitespace_inline(&mut self) {
        while let Some(c) = self.current() {
            if c == b' ' || c == b'\t' {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn tokenize(&mut self) -> Result<Vec<SpannedToken>, ConfigError> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current() {
            match c {
                b' ' | b'\t' | b'\r' => {
                    self.advance();
                }
                b'\n' => {
                    self.advance();
                    tokens.push(SpannedToken {
                        kind: TokenKind::NewLine,
                        line: self.line,
                        col: self.col,
                    });
                }
                b'#' => {
                    let comment = self.read_comment()?;
                    tokens.push(SpannedToken {
                        kind: TokenKind::Comment(comment),
                        line: self.line,
                        col: self.col,
                    })
                }
                b'[' => {
                    let (key, line, col) = self.read_table_header()?;
                    tokens.push(SpannedToken {
                        kind: TokenKind::TableHeader(key),
                        line,
                        col,
                    });
                }
                b'=' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::Equal,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'"' => {
                    let (s, line, col) = self.read_string()?;
                    tokens.push(SpannedToken {
                        kind: TokenKind::StringLit(s),
                        line,
                        col,
                    });
                }
                b'0'..=b'9' | b'-' if !is_alpha(self.peek().unwrap_or(b'\0')) => {
                    let (tok, line, col) = self.read_number()?;
                    tokens.push(SpannedToken {
                        kind: tok,
                        line,
                        col,
                    });
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let (word, line, col) = self.read_identifier();
                    tokens.push(SpannedToken {
                        kind: match word.as_str() {
                            "true" => TokenKind::Boolean(true),
                            "false" => TokenKind::Boolean(false),
                            other => TokenKind::Identifier(other.to_string()),
                        },
                        line,
                        col,
                    });
                }
                _ => {
                    let found = (c as char).to_string();
                    return Err(ConfigError::UnexpectedCharacter {
                        line: self.line,
                        col: self.col,
                        expected: "valid TOML syntax".to_string(),
                        found,
                    });
                }
            }
        }
        Ok(tokens)
    }
    fn read_comment(&mut self) -> Result<String, ConfigError> {
        self.advance();
        let mut result = String::new();
        while let Some(c) = self.current() {
            if c == b'\n' {
                break;
            }
            result.push(c as char);
            self.advance();
        }
        Ok(result)
    }
    fn read_table_header(&mut self) -> Result<(String, usize, usize), ConfigError> {
        let start_line = self.line;
        let start_col = self.col;

        self.advance();
        let mut dotted = String::new();
        while let Some(c) = self.current() {
            match c {
                b']' => {
                    self.advance();
                    return Ok((dotted, start_line, start_col));
                }
                b'.' => {
                    dotted.push('.');
                    self.advance();
                    self.skip_whitespace_inline();
                }
                b'\n' => {
                    return Err(ConfigError::UnterminatedString {
                        line: start_line,
                        col: start_col,
                    });
                }
                b' ' | b'\t' => {
                    self.advance();
                }
                _ => {
                    if is_alpha(c) || is_digit(c) || c == b'_' || c == b'-' {
                        dotted.push(c as char);
                        self.advance();
                    } else {
                        return Err(ConfigError::UnexpectedCharacter {
                            line: self.line,
                            col: self.col,
                            expected: "table key or ']'".to_string(),
                            found: (c as char).to_string(),
                        });
                    }
                }
            }
        }

        Err(ConfigError::UnterminatedString {
            line: start_line,
            col: start_col,
        })
    }
    fn read_string(&mut self) -> Result<(String, usize, usize), ConfigError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance();
        let mut result = String::new();
        loop {
            match self.current() {
                None => {
                    return Err(ConfigError::UnterminatedString {
                        line: start_line,
                        col: start_col,
                    });
                }
                Some(b'"') => {
                    self.advance();
                    return Ok((result, start_line, start_col));
                }
                Some(b'\\') => {
                    self.advance();
                    match self.current() {
                        Some(b'"') => {
                            result.push('"');
                            self.advance();
                        }
                        Some(b'\\') => {
                            result.push('\\');
                            self.advance();
                        }
                        Some(b'n') => {
                            result.push('\n');
                            self.advance();
                        }
                        Some(b'r') => {
                            result.push('\r');
                            self.advance();
                        }
                        Some(b't') => {
                            result.push('\t');
                            self.advance();
                        }
                        _ => {
                            let found = match self.current() {
                                Some(b) => (b as char).to_string(),
                                None => String::from("end of input"),
                            };
                            return Err(ConfigError::UnexpectedCharacter {
                                line: self.line,
                                col: self.col,
                                expected: "valid escape sequence".to_string(),
                                found,
                            });
                        }
                    }
                }
                Some(c) => {
                    result.push(c as char);
                    self.advance();
                }
            }
        }
    }
    fn read_number(&mut self) -> Result<(TokenKind, usize, usize), ConfigError> {
        let start_line = self.line;
        let start_col = self.col;
        let mut s = String::new();
        let mut has_dot = false;

        if let Some(b'-') = self.current() {
            s.push('-');
            self.advance();
        }

        loop {
            match self.current() {
                Some(c @ b'0'..=b'9') => {
                    s.push(c as char);
                    self.advance();
                }
                Some(b'.') => {
                    if has_dot {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "multiple decimal points".to_string(),
                        });
                    }
                    has_dot = true;
                    s.push('.');
                    self.advance();
                }
                _ => break,
            }
        }
        if has_dot {
            let value: f64 = match s.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ConfigError::InvalidNumber {
                        line: start_line,
                        col: start_col,
                        detail: format!("cannot parse '{}' as float", s),
                    });
                }
            };
            Ok((TokenKind::Float(value), start_line, start_col))
        } else {
            let value: i64 = match s.parse() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ConfigError::InvalidNumber {
                        line: start_line,
                        col: start_col,
                        detail: format!("cannot parse '{}' as integer", s),
                    });
                }
            };
            Ok((TokenKind::Integer(value), start_line, start_col))
        }
    }
    fn read_identifier(&mut self) -> (String, usize, usize) {
        let start_line = self.line;
        let start_col = self.col;
        let mut result = String::new();
        while let Some(c) = self.current() {
            if is_alpha(c) || is_digit(c) || c == b'_' || c == b'-' {
                result.push(c as char);
                self.advance();
            } else {
                break;
            }
        }
        (result, start_line, start_col)
    }
}
fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}
fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}
