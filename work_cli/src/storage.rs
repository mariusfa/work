use std::fs::{self};

pub trait FileOperations {
    fn read(&self) -> String;
    fn write(&mut self, content: &str);
}

pub struct RealFile {
    path: String,
}

pub struct MockFile {
    content: String,
}

impl FileOperations for RealFile {
    fn read(&self) -> String {
        fs::read_to_string(&self.path).unwrap()
    }

    fn write(&mut self, content: &str) {
        fs::write(&self.path, content.as_bytes()).unwrap();
    }
}

impl FileOperations for MockFile {
    fn read(&self) -> String {
        self.content.clone()
    }

    fn write(&mut self, content: &str) {
        self.content = content.to_string();
    }
}

impl MockFile {
    pub fn new() -> Self {
        MockFile { content: String::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_file_read_write() {
        let mut mock_file = MockFile { content: String::new() };

        // Test writing to the mock file
        mock_file.write("Hello, world!");
        assert_eq!(mock_file.read(), "Hello, world!");

        // Test overwriting the mock file
        mock_file.write("This is a test.");
        assert_eq!(mock_file.read(), "This is a test.");
    }
}
