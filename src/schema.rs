use crate::config_error::ConfigError;
use crate::field::{FieldSchema, FieldType, Schema};
use crate::toml_value::{TomlValue, toml_type_name};
use regex::Regex;
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
/*fn parse_array_schema(inner: &HashMap<String, TomlValue>) -> Result<FieldSchema, ConfigError> {

}*/
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
                required = match value {
                    TomlValue::Boolean(b) => *b,
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
                max = match value {
                    TomlValue::Integer(n) => Some(*n as usize),
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
            "minlen" => {
                min = match value {
                    TomlValue::Integer(n) => Some(*n as usize),
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
                description = match value {
                    TomlValue::String(s) => Some(s.clone()),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                };
            }
            "allowedvalues" => {
                allowed_values = match value {
                    TomlValue::Array(a) => Some(a.clone()),
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
                required = match value {
                    TomlValue::Boolean(b) => *b,
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
                description = match value {
                    TomlValue::String(s) => Some(s.clone()),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                };
            }
            "allowedvalues" => {
                allowed_values = match value {
                    TomlValue::Array(a) => Some(a.clone()),
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
                required = match value {
                    TomlValue::Boolean(b) => *b,
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
                min = match value {
                    TomlValue::Float(n) => Some(*n),
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
            "max" => {
                max = match value {
                    TomlValue::Float(n) => Some(*n),
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
                description = match value {
                    TomlValue::String(s) => Some(s.clone()),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                };
            }
            "allowedvalues" => {
                allowed_values = match value {
                    TomlValue::Array(a) => Some(a.clone()),
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
                required = match value {
                    TomlValue::Boolean(b) => *b,
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
                min = match value {
                    TomlValue::Integer(n) => Some(*n),
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
            "max" => {
                max = match value {
                    TomlValue::Integer(n) => Some(*n),
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
                description = match value {
                    TomlValue::String(s) => Some(s.clone()),
                    _ => {
                        return Err(ConfigError::ExpectedToken {
                            line: 0,
                            col: 0,
                            expected: "boolean".to_string(),
                            found: "non boolean".to_string(),
                        });
                    }
                };
            }
            "allowedvalues" => {
                allowed_values = match value {
                    TomlValue::Array(a) => Some(a.clone()),
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
        field_type: FieldType::Integer { min, max },
        required,
        default,
        allowed_values,
        description,
    })
}
pub fn parse_schema_internal(table: &HashMap<String, TomlValue>) -> Result<Schema, ConfigError> {
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
            Some(FieldType::Array(arr)) => {}
            Some(FieldType::Table(table)) => {}
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

fn validate_integer(
    min: &Option<i64>,
    max: &Option<i64>,
    allowed_values: &Option<Vec<TomlValue>>,
    key: &String,
    n: &i64,
) -> Result<(), ConfigError> {
    match min {
        Some(i) => {
            if n < i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!(">= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match max {
        Some(i) => {
            if n > i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!("<= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match allowed_values {
        Some(v) => {
            if !v.contains(&TomlValue::Integer(*n)) {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
        _ => {}
    }
    Ok(())
}
fn validate_float(
    min: &Option<f64>,
    max: &Option<f64>,
    allowed_values: &Option<Vec<TomlValue>>,
    key: &String,
    n: &f64,
) -> Result<(), ConfigError> {
    match min {
        Some(i) => {
            if n < &i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!(">= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match max {
        Some(i) => {
            if n > &i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!("<= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match allowed_values {
        Some(v) => {
            if !v.contains(&TomlValue::Float(*n)) {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}
fn validate_string(
    min: &Option<usize>,
    max: &Option<usize>,
    pattern: &Option<String>,
    allowed_values: &Option<Vec<TomlValue>>,
    key: &String,
    s: &String,
) -> Result<(), ConfigError> {
    match min {
        Some(i) => {
            if s.len() < *i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!(">= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match max {
        Some(i) => {
            if s.len() > *i {
                return Err(ConfigError::SchemaViolation {
                    line: 0,
                    key: key.clone(),
                    expected: format!("<= {}", i),
                    found: i.to_string(),
                });
            }
        }
        _ => {}
    }
    match pattern {
        Some(p) => {
            let re = Regex::new(&p).unwrap();
            if !re.is_match(&s) {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
        _ => {}
    }

    match allowed_values {
        Some(v) => {
            if !v.contains(&TomlValue::String(s.clone())) {
                return Err(ConfigError::ExpectedToken {
                    line: 0,
                    col: 0,
                    expected: "boolean".to_string(),
                    found: "non boolean".to_string(),
                });
            }
        }
        _ => {}
    }
    Ok(())
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
                    (FieldType::String { min, max, pattern }, TomlValue::String(s)) => {
                        validate_string(min, max, pattern, &field_schema.allowed_values, key, s)?;
                    }
                    (FieldType::Integer { min, max }, TomlValue::Integer(n)) => {
                        validate_integer(min, max, &field_schema.allowed_values, key, n)?;
                    }
                    (FieldType::Float { min, max }, TomlValue::Float(n)) => {
                        validate_float(min, max, &field_schema.allowed_values, key, n)?
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
