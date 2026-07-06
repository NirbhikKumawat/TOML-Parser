use crate::config_error::ConfigError;
use crate::field::{FieldType, Schema};
use crate::toml_value::TomlValue;
use regex::Regex;
use std::collections::HashMap;

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
                    (FieldType::Boolean, TomlValue::Boolean(_)) => {
                        //unreachable!()
                    }
                    (FieldType::Array(_), TomlValue::Array(_)) => {
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
