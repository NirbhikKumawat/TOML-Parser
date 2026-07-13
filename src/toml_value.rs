use crate::config_get_error::ConfigGetError;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TomlValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    DateTime(String),
    Array(Vec<TomlValue>),
    Table(HashMap<String, TomlValue>),
}

pub fn display(value: &TomlValue) -> String {
    match value {
        TomlValue::String(s) => format!("\"{}\"", s),
        TomlValue::Integer(i) => format!("{}", i),
        TomlValue::Float(f) => format!("{}", f),
        TomlValue::Boolean(b) => format!("{}", b),
        TomlValue::DateTime(d) => d.to_string(),
        TomlValue::Array(arr) => {
            let mut result = String::from("[");
            let mut first = true;
            for i in arr {
                if !first {
                    result.push_str(", ");
                }
                first = false;
                result.push_str(format!("{:?}", i).as_str());
            }
            result.push(']');
            result
        }
        TomlValue::Table(pairs) => {
            let mut result = String::from("{");
            let mut first = true;
            for (key, value) in pairs {
                if !first {
                    result.push_str(", ");
                }
                result.push_str(&format!("{}: {}", key, display(value)));
                first = false;
            }
            result.push('}');
            result
        }
    }
}
pub fn toml_type_name(value: &TomlValue) -> &'static str {
    match value {
        TomlValue::String(_) => "string",
        TomlValue::Integer(_) => "integer",
        TomlValue::Float(_) => "float",
        TomlValue::Boolean(_) => "boolean",
        TomlValue::Table(_) => "table",
        TomlValue::Array(_) => "array",
        TomlValue::DateTime(_) => "datetime",
    }
}

impl TomlValue {
    pub fn get(&self, key: &str) -> Result<&TomlValue, ConfigGetError> {
        match self {
            TomlValue::Table(map) => map.get(key).ok_or_else(|| ConfigGetError::MissingKey {
                key: key.to_string(),
            }),
            actual => Err(ConfigGetError::TypeMismatch {
                path: key.to_string(),
                expected: "table".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }
    pub fn get_at_index(&self, path: &str, index: usize) -> Result<&TomlValue, ConfigGetError> {
        match self.get(path)? {
            TomlValue::Array(arr) => {
                if index < arr.len() {
                    Ok(&arr[index])
                } else {
                    Err(ConfigGetError::IndexOutOfBounds {
                        path: format!("{}[{}]", path, index),
                        index,
                        len: arr.len(),
                    })
                }
            }
            actual => Err(ConfigGetError::TypeMismatch {
                path: path.to_string(),
                expected: "array".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }
    pub fn as_bool(&self, key: &str) -> Result<bool, ConfigGetError> {
        match self.get(key)? {
            TomlValue::Boolean(b) => Ok(*b),
            actual => Err(ConfigGetError::TypeMismatch {
                path: key.to_string(),
                expected: "boolean".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }
    pub fn as_float(&self, key: &str) -> Result<f64, ConfigGetError> {
        match self.get(key)? {
            TomlValue::Float(f) => Ok(*f),
            actual => Err(ConfigGetError::TypeMismatch {
                path: key.to_string(),
                expected: "float".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }
    pub fn as_int(&self, key: &str) -> Result<i64, ConfigGetError> {
        match self.get(key)? {
            TomlValue::Integer(i) => Ok(*i),
            actual => Err(ConfigGetError::TypeMismatch {
                path: key.to_string(),
                expected: "integer".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }

    pub fn as_str(&self, key: &str) -> Result<&str, ConfigGetError> {
        match self.get(key)? {
            TomlValue::String(s) => Ok(s.as_str()),
            actual => Err(ConfigGetError::TypeMismatch {
                path: key.to_string(),
                expected: "string".to_string(),
                found: toml_type_name(actual).to_string(),
            }),
        }
    }
}
