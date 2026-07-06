use crate::toml_value::TomlValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FieldType {
    Boolean,
    Integer {
        min: Option<i64>,
        max: Option<i64>,
    },
    Float {
        min: Option<f64>,
        max: Option<f64>,
    },
    String {
        min: Option<usize>,
        max: Option<usize>,
        pattern: Option<String>,
    },
    Array(Box<FieldType>),
    Table(Schema),
}
impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        let s = s.trim();
        match s {
            "string" => Some(FieldType::String {
                min: None,
                max: None,
                pattern: None,
            }),
            "integer" => Some(FieldType::Integer {
                min: None,
                max: None,
            }),
            "float" => Some(FieldType::Float {
                min: None,
                max: None,
            }),
            "boolean" => Some(FieldType::Boolean),
            _ => {
                if s.starts_with("array") && s.ends_with("]") {
                    if let Some(bracket) = s.find('[') {
                        let gap = s[5..bracket].trim();
                        if gap.is_empty() {
                            let inner = s[bracket + 1..s.len() - 1].trim();
                            if let Some(inner_type) = Self::from_str(inner) {
                                return Some(FieldType::Array(Box::new(inner_type)));
                            }
                        }
                    }
                }
                None
            }
        }
    }

    pub fn name(&self) -> String {
        match self {
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Integer { min: _, max: _ } => "integer".to_string(),
            FieldType::Float { min: _, max: _ } => "float".to_string(),
            FieldType::String {
                min: _,
                max: _,
                pattern: _,
            } => "string".to_string(),
            FieldType::Array(inner) => format!("array[{}]", inner.name()),
            FieldType::Table(_) => "table".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub(crate) field_type: FieldType,
    pub(crate) required: bool,
    pub(crate) default: Option<TomlValue>,
    pub(crate) allowed_values: Option<Vec<TomlValue>>,
    pub(crate) description: Option<String>,
}

pub type Schema = HashMap<String, FieldSchema>;
