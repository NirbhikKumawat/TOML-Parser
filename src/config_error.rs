#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(
        "unexpected character at line {line}, column {col}: expected {expected}, found '{found}'"
    )]
    UnexpectedCharacter {
        line: usize,
        col: usize,
        expected: String,
        found: String,
    },
    #[error("unterminated string starting at line {line}, column {col}")]
    UnterminatedString { line: usize, col: usize },
    #[error("invalid number at line {line}, column {col}: {detail}")]
    InvalidNumber {
        line: usize,
        col: usize,
        detail: String,
    },
    #[error("expected {expected} at line {line}, column {col}, found {found}")]
    ExpectedToken {
        line: usize,
        col: usize,
        expected: String,
        found: String,
    },
    #[error("missing value for key '{key}' at line {line}, column {col}")]
    MissingValue {
        line: usize,
        col: usize,
        key: String,
    },
    #[error("schema violation at line {line}: key '{key}' expected type {expected}, found {found}")]
    SchemaViolation {
        line: usize,
        key: String,
        expected: String,
        found: String,
    },
    #[error("missing required key '{key}'")]
    MissingRequiredKey { key: String },
    #[error("key '{key}' has value {value} which is outside range {min} to {max}")]
    ValueOutOfRange {
        key: String,
        value: String,
        min: i64,
        max: i64,
    },
    #[error("unknown key '{key}' at line {line}")]
    UnknownKey { key: String, line: usize },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
fn format_error(err: &ConfigError,source: &str) -> String {
    let line_num = match err {
        ConfigError::UnexpectedCharacter { line, .. } => *line,
        ConfigError::UnterminatedString { line, .. } => *line,
        ConfigError::InvalidNumber { line, .. } => *line,
        ConfigError::ExpectedToken { line, .. } => *line,
        ConfigError::MissingValue { line, .. } => *line,
        ConfigError::SchemaViolation { line, .. } => *line,
        ConfigError::UnknownKey { line, .. } => *line,
        _ => 0,
    };

    if line_num == 0 {
        return format!("Error: {}", err);
    }

    let lines: Vec<&str> = source.lines().collect();
    let line_content = match lines.get(line_num - 1) {
        Some(line) => *line,
        None => ""
    };

    format!(
        "Error: {}\n   |\n{:>2} | {}\n   |",
        err,
        line_num,
        line_content
    )

}
