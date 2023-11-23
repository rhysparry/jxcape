use crate::json::JsonValueCommand;
use crate::{read_all, ValueReader};
use clap::Args;
use std::io;
use std::io::BufRead;

#[derive(Args, Clone, Debug)]
pub struct StringArgs {
    /// Value to wrap in a JSON string
    #[arg(group = "input", required = true)]
    value: Option<String>,
    /// Read value from standard input.
    #[arg(long, group = "input")]
    from_stdin: bool,
}

impl ValueReader<String> for StringArgs {
    fn value(&self, alt_input: &mut dyn BufRead) -> io::Result<String> {
        if self.from_stdin {
            read_all(alt_input)
        } else {
            Ok(self.value.clone().unwrap_or_default())
        }
    }
}

impl JsonValueCommand for StringArgs {
    fn get_json_value(&self) -> serde_json::Value {
        match self.value.clone() {
            Some(v) => serde_json::Value::String(v),
            None => {
                let args: Box<dyn ValueReader<String>> = Box::new(self.clone());
                let value = args.value_stdin().unwrap();
                serde_json::Value::String(value.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::TestBuffer;
    #[test]
    fn test_string_args_value() {
        let args = StringArgs {
            value: Some("Hello, world!".to_string()),
            from_stdin: false,
        };
        let mut buffer = TestBuffer::empty();
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_string_args_value_from_std_in() {
        let args = StringArgs {
            value: None,
            from_stdin: true,
        };
        let mut buffer = TestBuffer::one_line("Hello, world!".to_string());
        let result = args.value(&mut buffer).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_string_args_to_json_value() {
        let args = StringArgs {
            value: Some("Hello, world!".to_string()),
            from_stdin: false,
        };
        let result = args.get_json_value();
        assert_eq!(
            result,
            serde_json::Value::String("Hello, world!".to_string())
        );
    }
}
