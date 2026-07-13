#[derive(Debug, Clone, thiserror::Error)]
pub enum ConfigGetError {
    #[error("required key '{key}' is missing")]
    MissingKey { key: String },
    #[error("key '{path}' expected a {expected}, but found a {found}")]
    TypeMismatch {
        path: String,
        expected: String,
        found: String,
    },
}

pub fn format_error(err: &ConfigGetError) -> String {
    format!("Err: {}", err)
}
