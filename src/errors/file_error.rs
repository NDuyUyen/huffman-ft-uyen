use std::fmt;

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
