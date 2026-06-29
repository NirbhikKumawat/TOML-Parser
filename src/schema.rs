use crate::config_error::ConfigError;
use crate::field::{FieldSchema, FieldType, Schema};
use crate::toml_value::{TomlValue, toml_type_name};

pub fn parse_schema(value: &TomlValue) -> Result<Schema, ConfigError> {
    let mut schema = Vec::new();
    let table = match value {
        TomlValue::Table(pairs) => pairs,
        _ => {
            return Err(ConfigError::UnexpectedCharacter {
                line: 0,
                col: 0,
                expected: "table".to_string(),
                found: "non-table value".to_string(),
            });
        }
    };
    for (key, value) in table {
        let inner = match value {
            TomlValue::Table(inner) => inner,
            _ => continue,
        };
        let mut field_type = FieldType::String;
        let mut required = false;
        let mut default_value: Option<TomlValue> = None;
        let mut min_val: Option<i64> = None;
        let mut max_val: Option<i64> = None;

        for (field_name, field_val) in inner {
            match field_name.as_str() {
                "type" => {
                    let type_str = match field_val {
                        TomlValue::String(s) => s.clone(),
                        _ => String::new(),
                    };
                    let ft = FieldType::from_str(&type_str);
                    match ft {
                        Some(t) => {
                            field_type = t;
                        }
                        None => {
                            return Err(ConfigError::SchemaViolation {
                                line: 0,
                                key: key.clone(),
                                expected: "valid type (string, integer, float, boolean)"
                                    .to_string(),
                                found: type_str,
                            });
                        }
                    }
                }
                "required" => {
                    required = match field_val {
                        TomlValue::Boolean(b) => *b,
                        _ => false,
                    };
                }
                "default" => {
                    default_value = Some(field_val.clone());
                }
                "min" => {
                    min_val = match field_val {
                        TomlValue::Integer(n) => Some(*n),
                        _ => None,
                    }
                }
                "max" => {
                    max_val = match field_val {
                        TomlValue::Integer(n) => Some(*n),
                        _ => None,
                    }
                }
                _ => {}
            }
        }
        schema.push((
            key.clone(),
            FieldSchema {
                field_type,
                required,
                default: default_value,
                min: min_val,
                max: max_val,
            },
        ));
    }

    Ok(schema)
}
pub fn flatten_table(table: &TomlValue, prefix: &str) -> Vec<(String, TomlValue)> {
    let pairs = match table {
        TomlValue::Table(pairs) => pairs,
        _ => return Vec::new(),
    };

    let mut result = Vec::new();
    for (key, value) in pairs {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{}.{}", prefix, key)
        };

        match value {
            TomlValue::Table(_) => {
                let nested = flatten_table(value, &full_key);
                for item in nested {
                    result.push(item);
                }
            }
            _ => {
                result.push((full_key, value.clone()));
            }
        }
    }

    result
}

fn find_line_for_key(source: &str, target_key: &str) -> usize {
    let short_key = if target_key.contains('.') {
        let mut parts = target_key.rsplit('.');
        match parts.next() {
            Some(part) => part,
            None => target_key,
        }
    } else {
        target_key
    };
    for (i, line) in source.lines().enumerate() {
        let trimmer = line.trim();
        if trimmer.starts_with('#') || trimmer.starts_with("[") || trimmer.is_empty() {
            continue;
        }
        if let Some(eq_pos) = trimmer.find('=') {
            let key = trimmer[..eq_pos].trim();
            if key == short_key {
                return i + 1;
            }
        }
    }
    0
}
pub fn validate(config: &TomlValue, schema: &Schema, source: &str) -> Result<(), Vec<ConfigError>> {
    let mut errors = Vec::new();
    let flat_config = flatten_table(config, "");

    for (key, field) in schema {
        let mut found_value: Option<&TomlValue> = None;
        for (k, v) in &flat_config {
            if k == key {
                found_value = Some(v);
                break;
            }
        }

        match found_value {
            None => {
                if field.required {
                    match &field.default {
                        Some(_) => {}
                        None => errors.push(ConfigError::MissingRequiredKey { key: key.clone() }),
                    }
                }
            }
            Some(value) => {
                let type_matches = match (&field.field_type, value) {
                    (FieldType::String, TomlValue::String(_)) => true,
                    (FieldType::Integer, TomlValue::Integer(_)) => true,
                    (FieldType::Float, TomlValue::Float(_)) => true,
                    (FieldType::Boolean, TomlValue::Boolean(_)) => true,
                    (FieldType::Float, TomlValue::Integer(_)) => true,
                    _ => false,
                };

                if !type_matches {
                    let line = find_line_for_key(source, key);
                    errors.push(ConfigError::SchemaViolation {
                        line,
                        key: key.clone(),
                        expected: field.field_type.name().to_string(),
                        found: toml_type_name(value).to_string(),
                    });
                }
                if let (TomlValue::Integer(n), Some(min)) = (value, field.min) {
                    if *n < min {
                        errors.push(ConfigError::ValueOutOfRange {
                            key: key.clone(),
                            value: n.to_string(),
                            min,
                            max: field.max.unwrap_or(0),
                        });
                    }
                }
                if let (TomlValue::Integer(n), Some(max)) = (value, field.max) {
                    if *n > max {
                        errors.push(ConfigError::ValueOutOfRange {
                            key: key.clone(),
                            value: n.to_string(),
                            min: field.min.unwrap_or(0),
                            max,
                        });
                    }
                }
            }
        }
    }

    for (key, _) in &flat_config {
        let mut is_known = false;
        for (k, _) in schema {
            if k == key {
                is_known = true;
                break;
            }
        }
        if !is_known {
            let line = find_line_for_key(source, key);
            errors.push(ConfigError::UnknownKey {
                key: key.clone(),
                line,
            })
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
