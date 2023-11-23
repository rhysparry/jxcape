use std::io::BufRead;

pub mod arrays;
pub mod cli;
pub mod json;
pub mod strings;
pub mod testing;

pub fn read_all(input: &mut dyn BufRead) -> std::io::Result<String> {
    let mut buffer = String::new();
    for l in input.lines() {
        buffer.push_str(l?.as_str());
        buffer.push('\n')
    }
    // Remove the excess final newline
    buffer.pop();
    Ok(buffer)
}

pub trait ValueReader<T> {
    fn value(&self, alt_input: &mut dyn BufRead) -> std::io::Result<T>;
}

impl<T> dyn ValueReader<T> {
    pub fn value_stdin(&self) -> std::io::Result<T> {
        let mut buffer = std::io::stdin().lock();
        self.value(&mut buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{TestBuffer, TestInput};

    #[test]
    fn test_read_all_from_one_line() {
        let input = TestInput::OneLine("Hello, world!".to_string());
        let mut buffer = TestBuffer::from(input);
        let result = read_all(&mut buffer).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_read_all_from_multiple_lines() {
        let input = TestInput::MultipleLines(vec![
            "Hello, world!".to_string(),
            "This is a test".to_string(),
        ]);
        let mut buffer = TestBuffer::from(input);
        let result = read_all(&mut buffer).unwrap();
        assert_eq!(result, "Hello, world!\nThis is a test");
    }
}
