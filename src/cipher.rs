pub fn encrypt(data: &Vec<u8>, key: &u8) -> Vec<u8> {
    data.iter().map(|val| val ^ key).collect()
}

pub fn decrypt(encrypted_data: &Vec<u8>, key: &u8) -> Vec<u8> {
    encrypted_data.iter().map(|val| val ^ key).collect()
}