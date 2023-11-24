use crate::json::JsonValueCommand;
use crate::ValueReader;
use clap::Args;
use serde_json::Map;
use std::io;
use std::io::BufRead;

#[derive(Args, Clone, Debug)]
pub struct ObjectArgs {
    /// Automatically detect JSON literals in input.
    #[arg(long)]
    auto: bool,
    /// Values to include in the object.
    #[arg(group = "input", required = true)]
    value: Vec<String>,
    /// Read values from standard input. One value per line.
    #[arg(long, group = "input")]
    from_stdin: bool,
    /// Return an empty object.
    #[arg(long, group = "input")]
    empty: bool,
    /// Separator between key and value.
    #[arg(long, short, default_value = "=")]
    separator: String,
}

struct KeyValue {
    key: String,
    value: Option<String>,
}

impl KeyValue {
    fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }

    fn from_str(s: &str, separator: &str) -> Self {
        let parts = s.split_once(separator);
        match parts {
            Some((key, value)) => Self::new(key.to_string(), Some(value.to_string())),
            None => Self::new(s.to_string(), None),
        }
    }
}

impl ObjectArgs {
    fn str_to_key_value(&self, s: &str) -> KeyValue {
        KeyValue::from_str(s, &self.separator)
    }
}

impl ValueReader<Vec<KeyValue>> for ObjectArgs {
    fn value(&self, alt_input: &mut dyn BufRead) -> io::Result<Vec<KeyValue>> {
        if self.from_stdin {
            let mut buffer = String::new();
            alt_input.read_to_string(&mut buffer)?;
            Ok(buffer.lines().map(|s| self.str_to_key_value(s)).collect())
        } else if self.empty {
            Ok(Vec::new())
        } else {
            Ok(self
                .value
                .iter()
                .map(|s| self.str_to_key_value(s))
                .collect())
        }
    }
}

impl JsonValueCommand for ObjectArgs {
    fn get_json_value(&self) -> serde_json::Value {
        use serde_json::Value;
        let args: Box<dyn ValueReader<Vec<KeyValue>>> = Box::new(self.clone());
        let mut map = Map::new();
        for kv in args.value_stdin().unwrap() {
            match kv.value {
                Some(value) => {
                    if self.auto {
                        let value = serde_json::from_str(&value).unwrap_or(Value::String(value));
                        map.insert(kv.key, value);
                    } else {
                        map.insert(kv.key, Value::String(value));
                    }
                }
                None => {
                    map.insert(kv.key, Value::Null);
                }
            }
        }

        Value::from(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TestBuffer;
    use serde_json::json;

    fn object_args(values: Vec<&str>) -> ObjectArgs {
        ObjectArgs {
            auto: false,
            value: values.into_iter().map(|s| s.to_string()).collect(),
            from_stdin: false,
            empty: false,
            separator: "=".to_string(),
        }
    }

    fn object_args_auto(values: Vec<&str>) -> ObjectArgs {
        ObjectArgs {
            auto: true,
            value: values.into_iter().map(|s| s.to_string()).collect(),
            from_stdin: false,
            empty: false,
            separator: "=".to_string(),
        }
    }

    #[test]
    fn test_key_value_from_str() {
        let kv = KeyValue::from_str("key=value", "=");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, Some("value".to_string()));
    }

    #[test]
    fn test_key_value_from_str_empty_value() {
        let kv = KeyValue::from_str("key=", "=");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, Some("".to_string()));
    }

    #[test]
    fn test_key_value_from_str_no_value() {
        let kv = KeyValue::from_str("key", "=");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, None);
    }

    #[test]
    fn test_key_value_from_str_repeated_separator() {
        let kv = KeyValue::from_str("key=value=other", "=");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, Some("value=other".to_string()));
    }

    #[test]
    fn test_key_value_from_str_long_separator() {
        let kv = KeyValue::from_str("key==value", "==");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, Some("value".to_string()));
    }

    #[test]
    fn test_key_value_from_str_long_separator_empty_value() {
        let kv = KeyValue::from_str("key==", "==");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, Some("".to_string()));
    }

    #[test]
    fn test_key_value_from_str_long_separator_no_value() {
        let kv = KeyValue::from_str("key", "==");
        assert_eq!(kv.key, "key");
        assert_eq!(kv.value, None);
    }

    #[test]
    fn test_object_args_single_value() {
        let args = object_args(vec!["key=value"]);
        let mut buffer = TestBuffer::empty();
        let values = args.value(&mut buffer);
        match values {
            Ok(values) => {
                assert_eq!(values.len(), 1);
                assert_eq!(values[0].key, "key");
                assert_eq!(values[0].value, Some("value".to_string()));
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_object_args_multiple_values() {
        let args = object_args(vec!["key=value", "other=thing"]);
        let mut buffer = TestBuffer::empty();
        let values = args.value(&mut buffer);
        match values {
            Ok(values) => {
                assert_eq!(values.len(), 2);
                assert_eq!(values[0].key, "key");
                assert_eq!(values[0].value, Some("value".to_string()));
                assert_eq!(values[1].key, "other");
                assert_eq!(values[1].value, Some("thing".to_string()));
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_object_args_json_value_single_value() {
        let args = object_args(vec!["key=value"]);
        let result = args.get_json_value();
        assert_eq!(
            result,
            json!({
                "key": "value",
            })
        );
    }

    #[test]
    fn test_object_args_json_value_multiple_values() {
        let args = object_args(vec!["key=value", "other=thing"]);
        let result = args.get_json_value();
        assert_eq!(
            result,
            json!({
                "key": "value",
                "other": "thing",
            })
        );
    }

    #[test]
    fn test_object_args_json_value_repeating_keys() {
        let args = object_args(vec!["key=value", "key=other"]);
        let result = args.get_json_value();
        assert_eq!(
            result,
            json!({
                "key": "other",
            })
        );
    }

    #[test]
    fn test_object_args_json_value_auto_json_parsing() {
        let args = object_args_auto(vec![
            "key=value",
            "other=true",
            "another=false",
            "number=9",
            "pi=3.14",
            "null=null",
            "empty=",
            "array=[]",
            "object={}",
            "array_with_value=[1]",
            "object_with_value={\"a\": 1}",
            "a_string=\"a string\"",
        ]);
        let result = args.get_json_value();
        assert_eq!(
            result,
            json!({
                "key": "value",
                "other": true,
                "another": false,
                "number": 9,
                "pi": 3.14,
                "null": null,
                "empty": "",
                "array": [],
                "object": {},
                "array_with_value": [1],
                "object_with_value": {"a": 1},
                "a_string": "a string",
            })
        );
    }

    #[test]
    fn test_object_args_json_value_empty_object() {
        let mut args = object_args(vec![]);
        args.empty = true;
        let result = args.get_json_value();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn test_object_args_from_stdin_multiple_lines() {
        let args = ObjectArgs {
            auto: false,
            value: Vec::new(),
            from_stdin: true,
            empty: false,
            separator: "=".to_string(),
        };
        let mut buffer =
            TestBuffer::multiple_lines(vec!["key=value".to_string(), "other=thing".to_string()]);
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].key, "key");
        assert_eq!(result[0].value, Some("value".to_string()));
        assert_eq!(result[1].key, "other");
        assert_eq!(result[1].value, Some("thing".to_string()));
    }
}
