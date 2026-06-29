#[derive(Debug, Clone, PartialEq)]
pub enum TomlValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<TomlValue>),
    Table(Vec<(String, TomlValue)>),
}

fn display(value: &TomlValue) -> String {
    match value {
        TomlValue::String(s) => format!("\"{}\"", s),
        TomlValue::Integer(i) => format!("{}", i),
        TomlValue::Float(f) => format!("{}", f),
        TomlValue::Boolean(b) => format!("{}", b),
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
            result.push_str("]");
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
            result.push_str("}");
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
    }
}