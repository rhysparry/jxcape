use crate::json::JsonValueCommand;
use crate::ValueReader;
use clap::Args;
use std::io;
use std::io::BufRead;

#[derive(Args, Clone, Debug)]
pub struct ArrayArgs {
    /// Automatically detect JSON literals in input.
    #[arg(long)]
    auto: bool,
    /// Values to include in the array.
    #[arg(group = "input", required = true)]
    value: Vec<String>,
    /// Read values from standard input. One value per line.
    #[arg(long, group = "input")]
    from_stdin: bool,
    /// Return an empty array.
    #[arg(long, group = "input")]
    empty: bool,
}

impl ValueReader<Vec<String>> for ArrayArgs {
    fn value(&self, alt_input: &mut dyn BufRead) -> io::Result<Vec<String>> {
        if self.from_stdin {
            let mut buffer = String::new();
            alt_input.read_to_string(&mut buffer)?;
            Ok(buffer.lines().map(|s| s.to_string()).collect())
        } else if self.empty {
            Ok(Vec::new())
        } else {
            Ok(self.value.clone())
        }
    }
}

impl JsonValueCommand for ArrayArgs {
    fn get_json_value(&self) -> serde_json::Value {
        use serde_json::Value;
        let args: Box<dyn ValueReader<Vec<String>>> = Box::new(self.clone());
        let value = args.value_stdin().unwrap();
        Value::Array(
            value
                .into_iter()
                .map(|v| {
                    if self.auto {
                        serde_json::from_str(&v).unwrap_or(Value::String(v))
                    } else {
                        Value::String(v)
                    }
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TestBuffer;
    use serde_json::json;

    fn array_args(values: Vec<&str>) -> ArrayArgs {
        ArrayArgs {
            auto: false,
            value: values.into_iter().map(|s| s.to_string()).collect(),
            from_stdin: false,
            empty: false,
        }
    }

    fn array_args_auto(values: Vec<&str>) -> ArrayArgs {
        ArrayArgs {
            auto: true,
            value: values.into_iter().map(|s| s.to_string()).collect(),
            from_stdin: false,
            empty: false,
        }
    }

    #[test]
    fn test_array_value_reader_from_std_in() {
        let args = ArrayArgs {
            auto: false,
            value: Vec::new(),
            from_stdin: true,
            empty: false,
        };
        let mut buffer = TestBuffer::multiple_lines(vec![
            "Hello, world!".to_string(),
            "This is a test".to_string(),
        ]);

        let result = args.value(&mut buffer).unwrap();
        assert_eq!(
            result,
            vec!["Hello, world!".to_string(), "This is a test".to_string(),]
        );
    }

    #[test]
    fn test_array_args_to_array_value() {
        let args = array_args(vec!["Hello, world!", "This is a test"]);
        let result = args.get_json_value();
        assert_eq!(result, json!(["Hello, world!", "This is a test",]));
    }

    #[test]
    fn test_array_args_auto_to_array_value() {
        let args = array_args_auto(vec![
            "Hello, world!",
            "true",
            "false",
            "9",
            "3.14",
            "null",
            "{}",
            "[]",
            "[1]",
            "{\"a\": 1}",
        ]);
        let result = args.get_json_value();
        assert_eq!(
            result,
            json!([
                "Hello, world!",
                true,
                false,
                9,
                3.14,
                null,
                {},
                [],
                [1],
                {"a": 1},
            ])
        );
    }
}
