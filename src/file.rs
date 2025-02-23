use std::fmt;
use std::{
    fs::{remove_file, File},
    io::{Read, Write},
};

// use crate::errors::file_error::FileError;

pub fn read_file_content(path: &str) -> Result<String, FileError> {
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(_) => Err(FileError::cannot_read_file(path)),
            }
        }
        Err(_) => Err(FileError::cannot_open_file(path)),
    }
}

pub fn write_file(path: &str, content: &str) -> Result<(), FileError> {
    match File::create_new(path) {
        Ok(mut file) => match file.write_all(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(FileError::cannot_write_file(path)),
        },
        Err(_) => Err(FileError::file_already_existed(path)),
    }
}

#[derive(Debug, PartialEq)]
pub struct FileError {
    pub msg: String,
}

impl FileError {
    pub fn cannot_open_file(path: &str) -> Self {
        let msg: String = format!("There is an error when trying to open file: {}", path);
        Self { msg: msg }
    }

    pub fn cannot_read_file(path: &str) -> Self {
        let msg: String = format!("There is an error when trying to read file: {}", path);
        Self { msg: msg }
    }

    pub fn cannot_write_file(path: &str) -> Self {
        let msg: String = format!("There is an error when trying to write file: {}", path);
        Self { msg: msg }
    }

    pub fn file_already_existed(path: &str) -> Self {
        let msg: String = format!("This file has already existed: {}", path);
        Self { msg: msg }
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", &self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_content_successful() {
        let path = "data/sample_1.txt".to_string();
        let result = read_file_content(&path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "This is the sample 1.");
    }

    #[test]
    fn test_read_file_content_failed() {
        let path = "data/invalid_sample_1.txt".to_string();
        let result = read_file_content(&path);

        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_content_successful() {
        let path = "data/output_1.txt".to_string();
        let content = "This is the outout 1.";
        let _ = remove_file(&path);
        let result = write_file(&path, content);

        assert!(result.is_ok());
    }

    #[test]
    fn test_write_file_content_failed() {
        let path = "data/sample_1.txt".to_string();
        let content = "This is the output 1.";
        let result = write_file(&path, content);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FileError::file_already_existed(&path));
    }
}
