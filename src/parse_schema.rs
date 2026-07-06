use crate::config_error::ConfigError;
use crate::field::{FieldSchema, FieldType, Schema};
use crate::toml_value::TomlValue;
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
fn parse_required(value: &TomlValue) -> Result<bool, ConfigError> {
    match value {
        TomlValue::Boolean(b) => Ok(*b),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_min_max_integer(value: &TomlValue) -> Result<Option<i64>, ConfigError> {
    match value {
        TomlValue::Integer(n) => Ok(Some(*n)),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_min_max_float(value: &TomlValue) -> Result<Option<f64>, ConfigError> {
    match value {
        TomlValue::Float(n) => Ok(Some(*n)),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_min_max_string(value: &TomlValue) -> Result<Option<usize>, ConfigError> {
    match value {
        TomlValue::Integer(n) => Ok(Some(*n as usize)),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_description_string(value: &TomlValue) -> Result<Option<String>, ConfigError> {
    match value {
        TomlValue::String(s) => Ok(Some(s.clone())),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_allowed_values(value: &TomlValue) -> Result<Option<Vec<TomlValue>>, ConfigError> {
    match value {
        TomlValue::Array(a) => Ok(Some(a.clone())),
        _ => Err(ConfigError::ExpectedToken {
            line: 0,
            col: 0,
            expected: "boolean".to_string(),
            found: "non boolean".to_string(),
        }),
    }
}
fn parse_string_schema(inner: &HashMap<String, TomlValue>) -> Result<FieldSchema, ConfigError> {
    let mut required: bool = false;
    let mut default: Option<TomlValue> = None;
    let mut allowed_values: Option<Vec<TomlValue>> = None;
    let mut description: Option<String> = None;
    let mut min: Option<usize> = None;
    let mut max: Option<usize> = None;
    let mut pattern: Option<String> = None;

    for (key, value) in inner {
        match key.as_str() {
            "type" => continue,
            "required" => {
                required = parse_required(value)?;
            }
            "default" => {
                default = match value {
                    TomlValue::String(s) => Some(TomlValue::String(s.clone())),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                }
            }
            "maxlen" => {
                max = parse_min_max_string(value)?;
            }
            "minlen" => {
                min = parse_min_max_string(value)?;
            }
            "description" => {
                description = parse_description_string(value)?;
            }
            "allowedvalues" => {
                allowed_values = parse_allowed_values(value)?;
            }
            "pattern" => {
                pattern = match value {
                    TomlValue::String(s) => Some(s.clone()),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                }
            }
            _ => {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
    }
    Ok(FieldSchema {
        field_type: FieldType::String { min, max, pattern },
        required,
        default,
        allowed_values,
        description,
    })
}
fn parse_boolean_schema(inner: &HashMap<String, TomlValue>) -> Result<FieldSchema, ConfigError> {
    let mut required: bool = false;
    let mut default: Option<TomlValue> = None;
    let mut description: Option<String> = None;
    let mut allowed_values: Option<Vec<TomlValue>> = None;

    for (key, value) in inner {
        match key.as_str() {
            "type" => continue,
            "required" => {
                required = parse_required(value)?;
            }
            "default" => {
                default = match value {
                    TomlValue::Boolean(b) => Some(TomlValue::Boolean(*b)),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                }
            }
            "description" => {
                description = parse_description_string(value)?;
            }
            "allowedvalues" => {
                allowed_values = parse_allowed_values(value)?;
            }
            _ => {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
    }
    Ok(FieldSchema {
        field_type: FieldType::Boolean,
        required,
        default,
        allowed_values,
        description,
    })
}
fn parse_float_schema(inner: &HashMap<String, TomlValue>) -> Result<FieldSchema, ConfigError> {
    let mut required: bool = false;
    let mut default: Option<TomlValue> = None;
    let mut allowed_values: Option<Vec<TomlValue>> = None;
    let mut description: Option<String> = None;
    let mut min: Option<f64> = None;
    let mut max: Option<f64> = None;

    for (key, value) in inner {
        match key.as_str() {
            "type" => continue,
            "required" => {
                required = parse_required(value)?;
            }
            "default" => {
                default = match value {
                    TomlValue::Float(n) => Some(TomlValue::Float(*n)),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                }
            }
            "min" => {
                min = parse_min_max_float(value)?;
            }
            "max" => {
                max = parse_min_max_float(value)?;
            }
            "description" => {
                description = parse_description_string(value)?;
            }
            "allowedvalues" => {
                allowed_values = parse_allowed_values(value)?;
            }
            _ => {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
    }
    Ok(FieldSchema {
        field_type: FieldType::Float { min, max },
        required,
        default,
        allowed_values,
        description,
    })
}
fn parse_integer_schema(inner: &HashMap<String, TomlValue>) -> Result<FieldSchema, ConfigError> {
    let mut required: bool = false;
    let mut default: Option<TomlValue> = None;
    let mut allowed_values: Option<Vec<TomlValue>> = None;
    let mut description: Option<String> = None;
    let mut min: Option<i64> = None;
    let mut max: Option<i64> = None;

    for (key, value) in inner {
        match key.as_str() {
            "type" => continue,
            "required" => {
                required = parse_required(value)?;
            }
            "default" => {
                default = match value {
                    TomlValue::Integer(n) => Some(TomlValue::Integer(*n)),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                }
            }
            "min" => {
                min = parse_min_max_integer(value)?;
            }
            "max" => {
                max = parse_min_max_integer(value)?;
            }
            "description" => {
                description = parse_description_string(value)?;
            }
            "allowedvalues" => {
                allowed_values = parse_allowed_values(value)?;
            }
            _ => {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
    }
    Ok(FieldSchema {
        field_type: FieldType::Integer { min, max },
        required,
        default,
        allowed_values,
        description,
    })
}
fn parse_schema_internal(table: &HashMap<String, TomlValue>) -> Result<Schema, ConfigError> {
    let mut schema = HashMap::new();

    for (key, value) in table {
        let inner = match value {
            TomlValue::Table(inner) => inner,
            _ => continue,
        };

        if !inner.contains_key("type") {
            let nested_schema = parse_schema_internal(inner)?;
            schema.insert(
                key.clone(),
                FieldSchema {
                    field_type: FieldType::Table(nested_schema),
                    required: true,
                    default: None,
                    allowed_values: None,
                    description: None,
                },
            );
            continue;
        }

        let type_str = match inner.get("type") {
            Some(TomlValue::String(s)) => s.clone(),
            _ => {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: "a string value for  'type'".to_string(),
                    found: "missing or invalid type".to_string(),
                });
            }
        };
        match FieldType::from_str(&type_str) {
            Some(FieldType::Integer { min: _, max: _ }) => {
                let value = parse_integer_schema(inner)?;
                schema.insert(key.clone(), value);
            }
            Some(FieldType::Float { min: _, max: _ }) => {
                let value = parse_float_schema(inner)?;
                schema.insert(key.clone(), value);
            }
            Some(FieldType::String {
                min: _,
                max: _,
                pattern: _,
            }) => {
                let value = parse_string_schema(inner)?;
                schema.insert(key.clone(), value);
            }
            Some(FieldType::Boolean) => {
                let value = parse_boolean_schema(inner)?;
                schema.insert(key.clone(), value);
            }
            Some(FieldType::Array(_)) => {}
            Some(FieldType::Table(_)) => {}
            None => {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: "valid type (e.g., string,integer,array[...])".to_string(),
                    found: type_str,
                });
            }
        };
    }
    Ok(schema)
}
