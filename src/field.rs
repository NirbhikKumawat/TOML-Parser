use crate::toml_value::TomlValue;

#[derive(Debug,Clone)]
pub enum FieldType {
    Boolean,
    Integer,
    Float,
    String,
}
impl FieldType {
    pub fn from_str(s: &str) -> Option<FieldType> {
        match s {
            "string" => Some(FieldType::String),
            "integer" => Some(FieldType::Integer),
            "float" => Some(FieldType::Float),
            "boolean" => Some(FieldType::Boolean),
            _ => None,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            FieldType::Boolean => "boolean",
            FieldType::Integer => "integer",
            FieldType::Float => "float",
            FieldType::String => "string",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub(crate) field_type: FieldType,
    pub(crate) required: bool,
    pub(crate) default: Option<TomlValue>,
    pub(crate) min: Option<i64>,
    pub(crate) max: Option<i64>,
}


pub type Schema = Vec<(String, FieldSchema)>;