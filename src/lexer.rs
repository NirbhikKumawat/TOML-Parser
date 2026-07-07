use crate::config_error::ConfigError;
use crate::token::{SpannedToken, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    pos: usize,
    line: usize,
    col: usize,
}
impl Lexer {
    pub fn new(input: &str) -> Self {
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
    fn peek_n(&self, n: usize) -> Option<u8> {
        if self.pos + n < self.input.len() {
            Some(self.input[self.pos + n])
        } else {
            None
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<SpannedToken>, ConfigError> {
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
                b'=' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::Equal,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b',' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::Comma,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'[' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::LBracket,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b']' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::RBracket,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'{' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::LBrace,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'}' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::RBrace,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'.' => {
                    tokens.push(SpannedToken {
                        kind: TokenKind::Dot,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                b'"' => {
                    if self.peek() == Some(b'"') && self.peek_n(2) == Some(b'"') {
                        let (s, line, col) = self.read_multiline_string()?;
                        tokens.push(SpannedToken {
                            kind: TokenKind::StringLit(s),
                            line,
                            col,
                        });
                    } else {
                        let (s, line, col) = self.read_string()?;
                        tokens.push(SpannedToken {
                            kind: TokenKind::StringLit(s),
                            line,
                            col,
                        });
                    }
                }
                b'\'' => {
                    if self.peek() == Some(b'\'') && self.peek_n(2) == Some(b'\'') {
                        let (s, line, col) = self.read_multiline_literal()?;
                        tokens.push(SpannedToken {
                            kind: TokenKind::StringLit(s),
                            line,
                            col,
                        });
                    } else {
                        let (s, line, col) = self.read_literal()?;
                        tokens.push(SpannedToken {
                            kind: TokenKind::StringLit(s),
                            line,
                            col,
                        });
                    }
                }
                b'0'..=b'9' | b'-' | b'+' => {
                    let (tok, line, col) = self.read_number()?;
                    tokens.push(SpannedToken {
                        kind: tok,
                        line,
                        col,
                    });
                }
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    let (word, line, col) = self.read_identifier();
                    if word == "inf" {
                        tokens.push(SpannedToken {
                            kind: TokenKind::Float(f64::INFINITY),
                            line,
                            col,
                        });
                    } else if word == "nan" {
                        tokens.push(SpannedToken {
                            kind: TokenKind::Float(f64::NAN),
                            line,
                            col,
                        });
                    } else {
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
    fn skip_newlines_in_multiline_string(&mut self) {
        if self.current() == Some(b'\n') {
            self.advance();
        } else if self.current() == Some(b'\r') && self.peek() == Some(b'\n') {
            self.advance();
            self.advance();
        };
    }
    fn read_multiline_string(&mut self) -> Result<(String, usize, usize), ConfigError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance();
        self.advance();
        self.advance();
        self.skip_newlines_in_multiline_string();

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
                    if self.peek() == Some(b'"') && self.peek_n(2) == Some(b'"') {
                        self.advance();
                        self.advance();
                        self.advance();
                        return Ok((result, start_line, start_col));
                    } else {
                        result.push('"');
                        self.advance();
                    }
                }
                Some(b'\\') => {
                    self.advance();
                    match self.current() {
                        Some(b'\n') | Some(b'\r') => {
                            while let Some(c) = self.current() {
                                if c == b'\n' || c == b'\t' || c == b' ' || c == b'\r' {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                        }
                        Some(b'\\') => {
                            result.push('\\');
                            self.advance();
                        }
                        Some(b'"') => {
                            result.push('"');
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
                            return Err(ConfigError::InvalidEscapeSequence {
                                line: self.line,
                                col: self.col,
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
        let mut has_exp = false;
        let mut is_nega = false;

        if let Some(b'0') = self.current() {
            match self.peek() {
                Some(b'x') => {
                    self.advance();
                    self.advance();
                    loop {
                        match self.current() {
                            Some(c @ b'0'..=b'9')
                            | Some(c @ b'a'..=b'f')
                            | Some(c @ b'A'..=b'F') => {
                                s.push(c as char);
                                self.advance();
                            }
                            Some(b'_') => {
                                match self.peek() {
                                    Some(b'0'..=b'9') | Some(b'a'..=b'f') | Some(b'A'..=b'F') => {}
                                    _ => {
                                        return Err(ConfigError::InvalidNumber {
                                            line: start_line,
                                            col: start_col,
                                            detail: "underscore should have numbers on both ends"
                                                .to_string(),
                                        });
                                    }
                                }
                                self.advance();
                            }
                            Some(c) => match c {
                                b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}' | b'#' => {
                                    break;
                                }
                                _ => {
                                    return Err(ConfigError::InvalidNumber {
                                        line: start_line,
                                        col: start_col,
                                        detail: format!(
                                            "invalid character '{}' in hexadecimal number",
                                            c as char
                                        ),
                                    });
                                }
                            },
                            None => break,
                        }
                    }
                    let value = i64::from_str_radix(s.as_str(), 16).map_err(|_| {
                        ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "invalid hexadecimal number".to_string(),
                        }
                    })?;
                    return Ok((TokenKind::Integer(value), start_line, start_col));
                }
                Some(b'b') => {
                    self.advance();
                    self.advance();
                    loop {
                        match self.current() {
                            Some(c @ b'0'..=b'1') => {
                                s.push(c as char);
                                self.advance();
                            }
                            Some(b'_') => {
                                match self.peek() {
                                    Some(b'0'..=b'1') => {}
                                    _ => {
                                        return Err(ConfigError::InvalidNumber {
                                            line: start_line,
                                            col: start_col,
                                            detail: "underscore should have numbers on both ends"
                                                .to_string(),
                                        });
                                    }
                                }
                                self.advance();
                            }
                            Some(c) => match c {
                                b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}' | b'#' => {
                                    break;
                                }
                                _ => {
                                    return Err(ConfigError::InvalidNumber {
                                        line: start_line,
                                        col: start_col,
                                        detail: format!(
                                            "invalid character '{}' in binary number",
                                            c as char
                                        ),
                                    });
                                }
                            },
                            None => break,
                        }
                    }
                    let value = i64::from_str_radix(s.as_str(), 2).map_err(|_| {
                        ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "invalid binary number".to_string(),
                        }
                    })?;
                    return Ok((TokenKind::Integer(value), start_line, start_col));
                }
                Some(b'o') => {
                    self.advance();
                    self.advance();
                    loop {
                        match self.current() {
                            Some(c @ b'0'..=b'7') => {
                                s.push(c as char);
                                self.advance();
                            }
                            Some(b'_') => {
                                match self.peek() {
                                    Some(b'0'..=b'7') => {}
                                    _ => {
                                        return Err(ConfigError::InvalidNumber {
                                            line: start_line,
                                            col: start_col,
                                            detail: "underscore should have numbers on both ends"
                                                .to_string(),
                                        });
                                    }
                                }
                                self.advance();
                            }
                            Some(c) => match c {
                                b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}' | b'#' => {
                                    break;
                                }
                                _ => {
                                    return Err(ConfigError::InvalidNumber {
                                        line: start_line,
                                        col: start_col,
                                        detail: format!(
                                            "invalid character '{}' in octal number",
                                            c as char
                                        ),
                                    });
                                }
                            },
                            None => break,
                        }
                    }
                    let value = i64::from_str_radix(s.as_str(), 8).map_err(|_| {
                        ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "invalid octal number".to_string(),
                        }
                    })?;
                    return Ok((TokenKind::Integer(value), start_line, start_col));
                }
                _ => {}
            }
        }

        if let Some(b'-') = self.current() {
            s.push('-');
            is_nega = true;
            self.advance();
        } else if let Some(b'+') = self.current() {
            s.push('+');
            self.advance();
        }

        match self.current() {
            Some(b'i') => {
                return if self.peek() == Some(b'n') && self.peek_n(2) == Some(b'f') {
                    self.advance();
                    self.advance();
                    self.advance();
                    if let Some(c @ b'a'..=b'z')
                    | Some(c @ b'A'..=b'Z')
                    | Some(c @ b'0'..=b'9')
                    | Some(c @ b'_') = self.current()
                    {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: format!("invalid character '{}' after inf", c as char),
                        });
                    }
                    let val = if is_nega {
                        f64::NEG_INFINITY
                    } else {
                        f64::INFINITY
                    };
                    Ok((TokenKind::Float(val), start_line, start_col))
                } else {
                    Err(ConfigError::InvalidNumber {
                        line: start_line,
                        col: start_col,
                        detail: "only nan and inf special characters are allowed".to_string(),
                    })
                };
            }
            Some(b'n') => {
                return if self.peek() == Some(b'a') && self.peek_n(2) == Some(b'n') {
                    self.advance();
                    self.advance();
                    self.advance();
                    if let Some(c @ b'a'..=b'z')
                    | Some(c @ b'A'..=b'Z')
                    | Some(c @ b'0'..=b'9')
                    | Some(c @ b'_') = self.current()
                    {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: format!("invalid character '{}' after nan", c as char),
                        });
                    }
                    let val = if is_nega { -f64::NAN } else { f64::NAN };
                    Ok((TokenKind::Float(val), start_line, start_col))
                } else {
                    Err(ConfigError::InvalidNumber {
                        line: start_line,
                        col: start_col,
                        detail: "only nan and inf special characters are allowed".to_string(),
                    })
                };
            }
            _ => {}
        }

        match self.current() {
            Some(c @ b'0'..=b'9') => {
                s.push(c as char);
                self.advance();
            }
            Some(b'_') => {
                return Err(ConfigError::InvalidNumber {
                    line: start_line,
                    col: start_col,
                    detail: "underscore should have numbers on both ends".to_string(),
                });
            }
            Some(b'.') => {
                return Err(ConfigError::InvalidNumber {
                    line: start_line,
                    col: start_col,
                    detail: "number cannot start with dot".to_string(),
                });
            }
            _ => {}
        }

        loop {
            match self.current() {
                Some(c @ b'0'..=b'9') => {
                    s.push(c as char);
                    self.advance();
                }
                Some(b'_') => {
                    match self.peek() {
                        Some(b'0'..=b'9') => {}
                        _ => {
                            return Err(ConfigError::InvalidNumber {
                                line: start_line,
                                col: start_col,
                                detail: "underscore should have numbers on both ends".to_string(),
                            });
                        }
                    }
                    self.advance();
                }
                Some(b'.') => {
                    if has_exp {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "fractional part cannot follow an exponent".to_string(),
                        });
                    }
                    if has_dot {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "multiple decimal points".to_string(),
                        });
                    }
                    match self.peek() {
                        Some(b'0'..=b'9') => {}
                        _ => {
                            return Err(ConfigError::InvalidNumber {
                                line: start_line,
                                col: start_col,
                                detail: "fractional part must start with a digit".to_string(),
                            });
                        }
                    }
                    has_dot = true;
                    s.push('.');
                    self.advance();
                }
                Some(c @ b'e') | Some(c @ b'E') => {
                    if has_exp {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: "multiple exponents".to_string(),
                        });
                    }
                    has_exp = true;
                    s.push(c as char);
                    self.advance();

                    if let Some(sign @ b'+') | Some(sign @ b'-') = self.current() {
                        s.push(sign as char);
                        self.advance();
                    }
                    match self.current() {
                        Some(c @ b'0'..=b'9') => {
                            s.push(c as char);
                            self.advance();
                        }
                        _ => {
                            return Err(ConfigError::InvalidNumber {
                                line: start_line,
                                col: start_col,
                                detail: "exponent part must start with a digit".to_string(),
                            });
                        }
                    }
                }
                Some(c) => match c {
                    b' ' | b'\t' | b'\n' | b'\r' | b',' | b']' | b'}' | b'#' => {
                        break;
                    }
                    _ => {
                        return Err(ConfigError::InvalidNumber {
                            line: start_line,
                            col: start_col,
                            detail: format!("invalid character '{}' in number", c as char),
                        });
                    }
                },
                None => break,
            }
        }
        if has_dot || has_exp {
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
    fn read_literal(&mut self) -> Result<(String, usize, usize), ConfigError> {
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
                Some(b'\n') => {
                    return Err(ConfigError::UnterminatedString {
                        line: start_line,
                        col: start_col,
                    });
                }
                Some(b'\'') => {
                    self.advance();
                    return Ok((result, start_line, start_col));
                }
                Some(c) => {
                    result.push(c as char);
                    self.advance();
                }
            }
        }
    }
    fn read_multiline_literal(&mut self) -> Result<(String, usize, usize), ConfigError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance();
        self.advance();
        self.advance();

        self.skip_newlines_in_multiline_string();

        let mut result = String::new();

        loop {
            match self.current() {
                None => {
                    return Err(ConfigError::UnterminatedString {
                        line: start_line,
                        col: start_col,
                    });
                }
                Some(b'\'') => {
                    if self.peek() == Some(b'\'') && self.peek_n(2) == Some(b'\'') {
                        self.advance();
                        self.advance();
                        self.advance();
                        return Ok((result, start_line, start_col));
                    } else {
                        result.push('\'');
                        self.advance();
                    }
                }
                Some(c) => {
                    result.push(c as char);
                    self.advance();
                }
            }
        }
    }
}
fn is_alpha(c: u8) -> bool {
    c.is_ascii_alphabetic()
}
fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}
