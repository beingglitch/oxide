use std::{fs, io, path::Path};

pub fn get_data(file: String) -> Result<Vec<u8>, io::Error>{
    let data = fs::read(file);
    return data
}

pub fn set_data(data: &Vec<u8>, file_name: Option<String>) -> Result<(), io::Error> {
    let file_name = file_name.unwrap_or("file.txt".to_string());

    let path = Path::new(&file_name);

    fs::write(path, data)?;
    Ok(())
}