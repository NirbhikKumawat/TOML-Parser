#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Equal,
    StringLit(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    TableHeader(String),
    Comment(String),
    NewLine,
}

#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub(crate) kind: TokenKind,
    pub(crate) line: usize,
    pub(crate) col: usize,
}
