use std::io;
use std::io::{BufRead, Read};

pub enum TestInput {
    OneLine(String),
    MultipleLines(Vec<String>),
}

pub struct TestBuffer {
    buffer: Vec<u8>,
    index: usize,
}

impl TestBuffer {
    pub fn empty() -> Self {
        Self {
            buffer: Vec::new(),
            index: 0,
        }
    }

    pub fn one_line(line: String) -> Self {
        Self::from(TestInput::OneLine(line))
    }

    pub fn multiple_lines(lines: Vec<String>) -> Self {
        Self::from(TestInput::MultipleLines(lines))
    }
}

impl From<TestInput> for TestBuffer {
    fn from(input: TestInput) -> Self {
        let mut buffer = Vec::new();
        match input {
            TestInput::OneLine(line) => buffer.extend(line.as_bytes()),
            TestInput::MultipleLines(lines) => {
                for line in lines {
                    buffer.extend(line.as_bytes());
                    buffer.push(b'\n');
                }
            }
        }
        Self { buffer, index: 0 }
    }
}

impl Read for TestBuffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = buf.len();
        let remaining = self.buffer.len() - self.index;
        let copy_len = if len > remaining { remaining } else { len };
        buf[..copy_len].copy_from_slice(&self.buffer[self.index..self.index + copy_len]);
        self.index += copy_len;
        Ok(copy_len)
    }
}

impl BufRead for TestBuffer {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(&self.buffer[self.index..])
    }

    fn consume(&mut self, amt: usize) {
        self.index += amt;
    }
}
