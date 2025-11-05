use std::fs;
use std::path::Path;
use crate::error::OxideError;

pub fn read_file(path: &Path) -> Result<Vec<u8>, OxideError> {
    fs::read(path)
        .map_err(|e| OxideError::FileReadError(e.to_string()))
}   

pub fn write_file(path: &Path, data: &[u8]) -> Result<(), OxideError> {
    fs::write(path, data)
        .map_err(|e| OxideError::FileWriteError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_write_and_read() {
        let path = PathBuf::from("test_temp.txt");
        let data = b"Hello, Oxide!";

        write_file(&path, data).expect("Failed to write");

        let read_data = read_file(&path).expect("Failed to read");

        assert_eq!(data, read_data.as_slice());

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_read_nonexistent_file() {
        let path = PathBuf::from("nonexistent_file_xyz.txt");
        let result = read_file(&path);

        assert!(result.is_err());

        if let Err(Oxide::FileReadError(msg)) = result {
            assert!(msg.contains("No such file") || msg.contains("cannot find"));
        } else {
            panic!("Expected FileReadError");
        }
    }
}