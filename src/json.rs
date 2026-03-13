pub trait JsonValueCommand {
    fn get_json_value(&self) -> serde_json::Value;
}

pub struct JsonPrettyPrinter;

impl JsonFormatter for JsonPrettyPrinter {
    fn format(&self, value: serde_json::Value) -> String {
        serde_json::to_string_pretty(&value).unwrap()
    }
}

pub struct JsonDefaultPrinter;

impl JsonFormatter for JsonDefaultPrinter {
    fn format(&self, value: serde_json::Value) -> String {
        serde_json::to_string(&value).unwrap()
    }
}

pub trait JsonFormatter {
    fn format(&self, value: serde_json::Value) -> String;
}

impl dyn JsonFormatter {
    pub fn print(&self, value: serde_json::Value) {
        println!("{}", self.format(value));
    }
}

pub fn auto_parse_values(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::String(s) => {
            serde_json::from_str(s).unwrap_or_else(|_| serde_json::Value::String(s.clone()))
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(auto_parse_values).collect())
        }
        serde_json::Value::Object(obj) => {
            let new_obj = obj
                .iter()
                .map(|(k, v)| (k.clone(), auto_parse_values(v)))
                .collect();
            serde_json::Value::Object(new_obj)
        }
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn auto_parse_integer_string() {
        let result = auto_parse_values(&json!("42"));
        assert_eq!(result, json!(42));
    }

    #[test]
    fn auto_parse_negative_integer_string() {
        let result = auto_parse_values(&json!("-7"));
        assert_eq!(result, json!(-7));
    }

    #[test]
    fn auto_parse_float_string() {
        let result = auto_parse_values(&json!("3.14"));
        assert_eq!(result, json!(3.14));
    }

    #[test]
    fn auto_parse_bool_true_string() {
        let result = auto_parse_values(&json!("true"));
        assert_eq!(result, json!(true));
    }

    #[test]
    fn auto_parse_bool_false_string() {
        let result = auto_parse_values(&json!("false"));
        assert_eq!(result, json!(false));
    }

    #[test]
    fn auto_parse_plain_string_unchanged() {
        let result = auto_parse_values(&json!("hello"));
        assert_eq!(result, json!("hello"));
    }

    #[test]
    fn auto_parse_non_string_values_unchanged() {
        assert_eq!(auto_parse_values(&json!(99)), json!(99));
        assert_eq!(auto_parse_values(&json!(true)), json!(true));
        assert_eq!(auto_parse_values(&json!(null)), json!(null));
    }

    #[test]
    fn auto_parse_array_coerces_elements() {
        let result = auto_parse_values(&json!(["1", "true", "hello"]));
        assert_eq!(result, json!([1, true, "hello"]));
    }

    #[test]
    fn auto_parse_object_coerces_values() {
        let result = auto_parse_values(&json!({ "count": "5", "flag": "false", "name": "bob" }));
        assert_eq!(result, json!({ "count": 5, "flag": false, "name": "bob" }));
    }

    #[test]
    fn auto_parse_object_keys_unchanged() {
        let input = json!({ "42": "value" });
        let result = auto_parse_values(&input);
        // Keys are never coerced — only values
        assert!(result.get("42").is_some());
        assert_eq!(result["42"], json!("value"));
    }

    #[test]
    fn auto_parse_nested_object() {
        let result = auto_parse_values(&json!({ "outer": { "inner": "99" } }));
        assert_eq!(result, json!({ "outer": { "inner": 99 } }));
    }

    #[test]
    fn auto_parse_inf_string_stays_string() {
        // "inf" parses as f64::INFINITY, but JSON has no infinity literal.
        // from_f64 returns None for non-finite values, so the string is preserved.
        let result = auto_parse_values(&json!("inf"));
        assert_eq!(result, json!("inf"));
    }

    #[test]
    fn auto_parse_negative_inf_string_stays_string() {
        let result = auto_parse_values(&json!("-inf"));
        assert_eq!(result, json!("-inf"));
    }

    #[test]
    fn auto_parse_nan_string_stays_string() {
        let result = auto_parse_values(&json!("NaN"));
        assert_eq!(result, json!("NaN"));
    }

    #[test]
    fn auto_parse_infinity_string_stays_string() {
        let result = auto_parse_values(&json!("infinity"));
        assert_eq!(result, json!("infinity"));
    }

    #[test]
    fn auto_parse_json_object_literal_string() {
        let result = auto_parse_values(&json!(r#"{"key":"value"}"#));
        assert_eq!(result, json!({"key": "value"}));
    }

    #[test]
    fn auto_parse_json_array_literal_string() {
        let result = auto_parse_values(&json!(r#"[1,2,3]"#));
        assert_eq!(result, json!([1, 2, 3]));
    }

    #[test]
    fn auto_parse_null_literal_string() {
        let result = auto_parse_values(&json!("null"));
        assert_eq!(result, json!(null));
    }

    #[test]
    fn auto_parse_nested_json_object_literal_string() {
        let result = auto_parse_values(&json!(r#"{"outer":{"inner":42}}"#));
        assert_eq!(result, json!({"outer": {"inner": 42}}));
    }

    #[test]
    fn auto_parse_json_array_of_strings_literal() {
        let result = auto_parse_values(&json!(r#"["a","b","c"]"#));
        assert_eq!(result, json!(["a", "b", "c"]));
    }
}
