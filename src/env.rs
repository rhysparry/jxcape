use crate::json::JsonValueCommand;
use crate::non_empty_separator;
use crate::ValueReader;
use clap::Args;
use serde_json::{Map, Value};
use std::env;
use std::io;
use std::io::BufRead;

#[derive(Args, Clone, Debug, Default)]
pub struct EnvArgs {
    /// Automatically detect JSON literals in environment variable values.
    #[arg(long)]
    auto: bool,
    /// Reads the values from standard input, formatted in .env syntax
    #[arg(long)]
    from_stdin: bool,
    /// Expands the Path environment variable into an array
    #[arg(long)]
    expand_path: bool,
    /// Separator to use when expanding the Path variable. Defaults to the platform-specific separator.
    #[arg(long, requires = "expand_path", value_parser = non_empty_separator)]
    path_separator: Option<String>,
}

fn get_current_env_as_json() -> Value {
    let mut env_map = Map::new();
    for (key, value) in env::vars() {
        env_map.insert(key, Value::String(value));
    }
    Value::Object(env_map)
}

fn read_env_data(source: &str) -> io::Result<Value> {
    let env = env_file_reader::read_str(source)?;
    let mut env_map = Map::new();
    for (key, value) in env {
        env_map.insert(key, Value::String(value));
    }
    Ok(Value::Object(env_map))
}

impl ValueReader<Value> for EnvArgs {
    fn value(&self, alt_input: &mut dyn BufRead) -> io::Result<Value> {
        if self.from_stdin {
            let mut buffer = String::new();
            alt_input.read_to_string(&mut buffer)?;
            read_env_data(&buffer)
        } else {
            Ok(get_current_env_as_json())
        }
    }
}

impl JsonValueCommand for EnvArgs {
    fn get_json_value(&self) -> Value {
        let args: Box<dyn ValueReader<Value>> = Box::new(self.clone());
        let mut result = args.value_stdin().unwrap();
        if self.auto {
            result = crate::json::auto_parse_values(&result);
        }

        self.expand_path_variable(&mut result);

        result
    }
}

impl EnvArgs {
    fn get_path_separator(&self) -> &str {
        self.path_separator
            .as_deref()
            .unwrap_or(if cfg!(windows) { ";" } else { ":" })
    }

    fn is_path_variable(&self, key: &str) -> bool {
        key.eq_ignore_ascii_case("PATH")
    }

    fn expand_path_variable(&self, json_value: &mut Value) {
        if self.expand_path {
            let separator = self.get_path_separator();
            if let Value::Object(map) = json_value {
                // Iterate over the map, find path variables and expand them
                for (key, value) in map.iter_mut() {
                    if self.is_path_variable(key) {
                        if let Value::String(s) = value {
                            let parts: Vec<Value> = s
                                .split(separator)
                                .map(|part| Value::String(part.to_string()))
                                .collect();
                            *value = Value::Array(parts);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TestBuffer;
    use serde_json::json;

    #[derive(clap::Parser, Debug)]
    struct EnvArgsParser {
        #[command(flatten)]
        args: EnvArgs,
    }

    fn env_args_from_stdin() -> EnvArgs {
        let mut args = EnvArgs::default();
        args.from_stdin = true;
        args
    }

    fn env_args_from_env() -> EnvArgs {
        EnvArgs::default()
    }

    fn env_args_auto_from_stdin() -> EnvArgs {
        let mut args = EnvArgs::default();
        args.auto = true;
        args.from_stdin = true;
        args
    }

    #[test]
    fn from_stdin_parses_single_key_value_pair() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::one_line("KEY=value".to_string());
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, json!({ "KEY": "value" }));
    }

    #[test]
    fn from_stdin_parses_multiple_key_value_pairs() {
        let args = env_args_from_stdin();
        let mut buffer =
            TestBuffer::multiple_lines(vec!["FOO=bar".to_string(), "BAZ=qux".to_string()]);
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, json!({ "FOO": "bar", "BAZ": "qux" }));
    }

    #[test]
    fn from_stdin_parses_quoted_values() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::one_line(r#"GREETING="hello world""#.to_string());
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, json!({ "GREETING": "hello world" }));
    }

    #[test]
    fn from_stdin_empty_input_produces_empty_object() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::empty();
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, json!({}));
    }

    #[test]
    fn from_env_returns_json_object() {
        let args = env_args_from_env();
        let mut buffer = TestBuffer::empty();
        let result = args.value(&mut buffer).unwrap();
        assert!(result.is_object());
    }

    #[test]
    fn from_env_includes_known_env_var() {
        let (key, expected) = std::env::vars()
            .next()
            .expect("test environment must have at least one environment variable");
        let args = env_args_from_env();
        let mut buffer = TestBuffer::empty();
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result[&key], json!(expected));
    }

    #[test]
    fn from_stdin_values_are_strings() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::one_line("NUM=42".to_string());
        let result = args.value(&mut buffer).unwrap();
        // Values are always strings, never coerced to numbers
        assert_eq!(result, json!({ "NUM": "42" }));
    }

    #[test]
    fn auto_coerces_integer_value_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("NUM=42".to_string());
        // value() returns the raw string; caller must apply auto_parse_values
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result, json!({ "NUM": 42 }));
    }

    #[test]
    fn auto_coerces_bool_value_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("FLAG=true".to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result, json!({ "FLAG": true }));
    }

    #[test]
    fn auto_coerces_float_value_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("RATIO=1.5".to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result, json!({ "RATIO": 1.5 }));
    }

    #[test]
    fn auto_leaves_plain_string_unchanged_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("NAME=alice".to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result, json!({ "NAME": "alice" }));
    }

    #[test]
    fn auto_coerces_env_var_via_get_json_value() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("JXCAPE_AUTO_TEST_INT=100".to_string());
        let result = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&result);
        assert_eq!(result["JXCAPE_AUTO_TEST_INT"], json!(100));
    }

    #[test]
    fn no_auto_env_var_stays_string_via_get_json_value() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::one_line("NUM=200".to_string());
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result["NUM"], json!("200"));
    }

    #[test]
    fn auto_coerces_json_object_literal_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer =
            TestBuffer::one_line(r#"CONFIG='{"host":"localhost","port":5432}'"#.to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result["CONFIG"], json!({"host": "localhost", "port": 5432}));
    }

    #[test]
    fn auto_coerces_json_array_literal_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line(r#"TAGS='["foo","bar","baz"]'"#.to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result["TAGS"], json!(["foo", "bar", "baz"]));
    }

    #[test]
    fn auto_coerces_null_literal_from_stdin() {
        let args = env_args_auto_from_stdin();
        let mut buffer = TestBuffer::one_line("EMPTY=null".to_string());
        let raw = args.value(&mut buffer).unwrap();
        let result = crate::json::auto_parse_values(&raw);
        assert_eq!(result["EMPTY"], json!(null));
    }

    #[test]
    fn auto_json_object_literal_without_auto_stays_string() {
        let args = env_args_from_stdin();
        let mut buffer = TestBuffer::one_line(r#"CONFIG='{"host":"localhost"}'"#.to_string());
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result["CONFIG"], json!(r#"{"host":"localhost"}"#));
    }

    // --- expand_path tests ---

    fn env_args_expand_path() -> EnvArgs {
        let mut args = EnvArgs::default();
        args.expand_path = true;
        args
    }

    fn env_args_expand_path_with_separator(sep: &str) -> EnvArgs {
        let mut args = EnvArgs::default();
        args.expand_path = true;
        args.from_stdin = true;
        args.path_separator = Some(sep.to_string());
        args
    }

    #[test]
    fn expand_path_disabled_leaves_path_as_string() {
        let args = env_args_from_stdin();
        let mut json_value = json!({ "PATH": "/usr/bin:/bin" });
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value, json!({ "PATH": "/usr/bin:/bin" }));
    }

    #[test]
    fn expand_path_splits_on_default_separator() {
        let args = env_args_expand_path();
        let mut json_value = json!({ "PATH": "/usr/bin:/bin:/usr/local/bin" });
        args.expand_path_variable(&mut json_value);
        let expected = json!({ "PATH": ["/usr/bin", "/bin", "/usr/local/bin"] });
        assert_eq!(json_value, expected);
    }

    #[test]
    fn expand_path_splits_on_custom_separator() {
        let args = env_args_expand_path_with_separator(";");
        let mut json_value = json!({ "PATH": "C:\\Windows;C:\\Windows\\System32" });
        args.expand_path_variable(&mut json_value);
        let expected = json!({ "PATH": ["C:\\Windows", "C:\\Windows\\System32"] });
        assert_eq!(json_value, expected);
    }

    #[test]
    fn expand_path_single_entry_produces_single_element_array() {
        let args = env_args_expand_path();
        let mut json_value = json!({ "PATH": "/usr/bin" });
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value, json!({ "PATH": ["/usr/bin"] }));
    }

    #[test]
    fn expand_path_is_case_insensitive() {
        let args = env_args_expand_path();
        let mut json_value = json!({ "path": "/usr/bin:/bin" });
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value, json!({ "path": ["/usr/bin", "/bin"] }));
    }

    #[test]
    fn expand_path_mixed_case_is_case_insensitive() {
        let args = env_args_expand_path();
        let mut json_value = json!({ "Path": "/usr/bin:/bin" });
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value, json!({ "Path": ["/usr/bin", "/bin"] }));
    }

    #[test]
    fn expand_path_does_not_affect_other_keys() {
        let args = env_args_expand_path();
        let mut json_value = json!({ "PATH": "/usr/bin:/bin", "HOME": "/home/user" });
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value["HOME"], json!("/home/user"));
    }

    #[test]
    fn expand_path_from_stdin_splits_path_into_array() {
        // Exercises value() + expand_path_variable() directly, which is the same
        // logic get_json_value() composes. get_json_value() cannot accept a
        // TestBuffer as it reads real stdin via value_stdin().
        let args = env_args_expand_path_with_separator(":");
        let mut buffer = TestBuffer::one_line("PATH=/usr/bin:/bin".to_string());
        let mut json_value = args.value(&mut buffer).unwrap();
        args.expand_path_variable(&mut json_value);
        assert_eq!(json_value["PATH"], json!(["/usr/bin", "/bin"]));
    }

    #[test]
    fn empty_path_separator_is_rejected() {
        use clap::Parser;
        let result = EnvArgsParser::try_parse_from(["cmd", "--expand-path", "--path-separator="]);
        assert!(result.is_err());
    }
}
