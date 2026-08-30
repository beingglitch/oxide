use std::{fs, path::Path};

use crate::errors::OxideError;

pub fn get_data(file: String) -> Result<Vec<u8>, OxideError>{
    fs::read(file).map_err(OxideError::ReadFailed)
}

pub fn set_data(data: &Vec<u8>, file_name: Option<String>) -> Result<(), OxideError> {
    let file_name = file_name.unwrap_or("file.txt".to_string());

    let path = Path::new(&file_name);

    fs::write(path, data).map_err(OxideError::WriteFailed)
}