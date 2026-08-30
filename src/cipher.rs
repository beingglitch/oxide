const MAGIC: &[u8; 5] = b"OXIDE";

pub fn encrypt(data: &Vec<u8>, key: &str) -> Vec<u8> {
    let new_data: Vec<u8> = MAGIC.iter().chain(data.iter()).copied().collect();
    let keystream = generate_keystream(key, new_data.len());
    new_data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect()
}
pub fn decrypt(encrypted_data: &Vec<u8>, key: &str) -> Result<Vec<u8>, &'static str> {
    let keystream = generate_keystream(key, encrypted_data.len());
    let decrypted_data: Vec<u8> = encrypted_data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect();

    // Check password or written decrypted file
    if decrypted_data[0..MAGIC.len()] == *MAGIC {
        Ok(decrypted_data[MAGIC.len()..].to_vec())
    } else {
        Err("Wrong Password")
    }
}


pub fn generate_keystream(key: &str, length: usize) -> Vec<u8> {
    let key_bytes = key.as_bytes();

    // key_bytes.iter().cycle().take(length).copied() -> lazy stages; copied gives ownership from references
    // .collec() -> consuming adaptor
    let keystream = key_bytes.iter().cycle().take(length).copied().collect();

    keystream
}