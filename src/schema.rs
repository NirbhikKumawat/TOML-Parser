use crate::config_error::ConfigError;
use crate::field::{FieldSchema, FieldType, Schema};
use crate::toml_value::{TomlValue, toml_type_name};
use std::collections::HashMap;

pub fn parse_schema(value: &TomlValue) -> Result<Schema, ConfigError> {
    match value {
        TomlValue::Table(pairs) => parse_schema_internal(pairs),
        _ => Err(ConfigError::UnexpectedCharacter {
            line: 0,
            col: 0,
            expected: "table".to_string(),
            found: "non-table value".to_string(),
        }),
    }
}
pub fn parse_schema_internal(table: &HashMap<String, TomlValue>) -> Result<Schema, ConfigError> {
    let mut schema = HashMap::new();

    for (key, value) in table {
        let inner = match value {
            TomlValue::Table(inner) => inner,
            _ => continue,
        };

        if inner.contains_key("type") {
            let mut field_type = FieldType::String;
            let mut required = false;
            let mut default_value: Option<TomlValue> = None;
            let mut min_val: Option<i64> = None;
            let mut max_val: Option<i64> = None;
            let mut min_length: Option<usize> = None;
            let mut max_length: Option<usize> = None;
            let mut allowed_values: Option<Vec<TomlValue>> = None;
            let mut pattern: Option<String> = None;
            let mut description: Option<String> = None;

            for (field_name, field_value) in inner {
                match field_name.as_str() {
                    "type" => {
                        let type_str = match field_value {
                            TomlValue::String(s) => s.clone(),
                            _ => String::new(),
                        };
                        match FieldType::from_str(&type_str) {
                            Some(t) => {
                                field_type = t;
                            }
                            None => {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: "valid type (e.g., string,integer,array[...])"
                                        .to_string(),
                                    found: type_str,
                                });
                            }
                        }
                    }
                    "required" => {
                        required = match field_value {
                            TomlValue::Boolean(b) => *b,
                            _ => false,
                        }
                    }
                    "default" => {
                        default_value = Some(field_value.clone());
                    }
                    "min" => {
                        min_val = match field_value {
                            TomlValue::Integer(n) => Some(*n),
                            _ => None,
                        }
                    }
                    "max" => {
                        max_val = match field_value {
                            TomlValue::Integer(n) => Some(*n),
                            _ => None,
                        }
                    }
                    "min_length" => {
                        min_length = match field_value {
                            TomlValue::Integer(n) if *n >= 0 => Some(*n as usize),
                            _ => None,
                        }
                    }
                    "max_length" => {
                        max_length = match field_value {
                            TomlValue::Integer(n) if *n >= 0 => Some(*n as usize),
                            _ => None,
                        }
                    }
                    "allowed_values" => {
                        allowed_values = match field_value {
                            TomlValue::Array(arr) => Some(arr.clone()),
                            _ => None,
                        }
                    }
                    "pattern" => {
                        pattern = match field_value {
                            TomlValue::String(s) => Some(s.clone()),
                            _ => None,
                        }
                    }
                    "description" => {
                        description = match field_value {
                            TomlValue::String(s) => Some(s.clone()),
                            _ => None,
                        }
                    }
                    _ => {}
                }
            }
            schema.insert(
                key.clone(),
                FieldSchema {
                    field_type,
                    required,
                    default: default_value,
                    min: min_val,
                    max: max_val,
                    min_len: min_length,
                    max_len: max_length,
                    allowed_values,
                    pattern,
                    description,
                },
            );
        } else {
            let nested_schema = parse_schema_internal(inner)?;
            schema.insert(
                key.clone(),
                FieldSchema {
                    field_type: FieldType::Table(nested_schema),
                    required: true,
                    default: None,
                    min: None,
                    max: None,
                    min_len: None,
                    max_len: None,
                    allowed_values: None,
                    pattern: None,
                    description: None,
                },
            );
        }
    }

    Ok(schema)
}

pub fn validate(schema: &Schema, config: &HashMap<String, TomlValue>) -> Result<(), ConfigError> {
    for (key, field_schema) in schema {
        match config.get(key) {
            None => {
                if field_schema.required {
                    return Err(ConfigError::MissingValue {
                        line: 0,
                        col: 0,
                        key: key.clone(),
                    });
                }
            }
            Some(value) => {
                match (&field_schema.field_type, value) {
                    (FieldType::String, TomlValue::String(s)) => {
                        if let Some(min) = field_schema.min_len {
                            if s.len() < min {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!(">= {}", min),
                                    found: s.len().to_string(),
                                });
                            }
                        }
                        if let Some(max) = field_schema.max_len {
                            if s.len() > max {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!("<= {}", max),
                                    found: s.len().to_string(),
                                });
                            }
                        }
                    }
                    (FieldType::Integer, TomlValue::Integer(i)) => {
                        if let Some(min) = field_schema.min {
                            if *i < min {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!(">= {}", min),
                                    found: i.to_string(),
                                });
                            }
                        }
                        if let Some(max) = field_schema.max {
                            if *i > max {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!("<= {}", max),
                                    found: i.to_string(),
                                });
                            }
                        }
                    }
                    (FieldType::Float, TomlValue::Float(i)) => {
                        if let Some(min) = field_schema.min {
                            if *i < min as f64 {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!(">= {}", min),
                                    found: i.to_string(),
                                });
                            }
                        }
                        if let Some(max) = field_schema.max {
                            if *i > max as f64 {
                                return Err(ConfigError::SchemaViolation {
                                    line: 0,
                                    key: key.clone(),
                                    expected: format!("<= {}", max),
                                    found: i.to_string(),
                                });
                            }
                        }
                    }
                    (FieldType::Boolean, TomlValue::Boolean(b)) => {
                        //unreachable!()
                    }
                    (FieldType::Array(inner_type), TomlValue::Array(arr)) => {
                        //unreachable!()
                    }
                    (FieldType::Table(inner_type), TomlValue::Table(table)) => {
                        validate(inner_type, table)?;
                    }
                    (expected_type, actual_type) => {
                        return Err(ConfigError::SchemaViolation {
                            line: 0,
                            key: key.clone(),
                            expected: expected_type.name(),
                            found: format!("{:?}", actual_type),
                        });
                    }
                }
            }
        }
    }
    Ok(())
}
