use crate::errors::OxideError;

const MAGIC: &[u8; 5] = b"OXIDE";

pub fn encrypt(data: &Vec<u8>, key: &str) -> Vec<u8> {
    let new_data: Vec<u8> = MAGIC.iter().chain(data.iter()).copied().collect();
    let keystream = generate_keystream(key.as_bytes(), new_data.len());
    new_data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect()
}
pub fn decrypt(encrypted_data: &Vec<u8>, key: &str) -> Result<Vec<u8>, OxideError> {
    let keystream = generate_keystream(key.as_bytes(), encrypted_data.len());
    let decrypted_data: Vec<u8> = encrypted_data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect();

    // Check password or written decrypted file
    if decrypted_data[0..MAGIC.len()] == *MAGIC {
        Ok(decrypted_data[MAGIC.len()..].to_vec())
    } else {
        Err(OxideError::WrongPassword)
    }
}


pub fn generate_keystream(seed: &[u8], length: usize) -> Vec<u8> {
    // key_bytes.iter().cycle().take(length).copied() -> lazy stages; copied gives ownership from references
    // .collec() -> consuming adaptor
    let keystream = seed.iter().cycle().take(length).copied().collect();

    keystream
}

const SEED_SIZE: usize = 32;

// Old logic (had a rotation-mod-8 collision, see: seed[0] == seed[8]):
// for i in 0..SEED_SIZE {
//     for byte in password {
//         accumulator[i] = accumulator[i].rotate_left((i + 1) as u32) ^ byte;
//     }
// }
// But this was causing a issue i.e., every i/8th rotation is same means 1, 9, 17 produces 
// same number after rotation when rotating number is of 8 bit

pub fn derive_seed(password: &[u8]) -> [u8; SEED_SIZE] {

    let mut accumulator:[u8; SEED_SIZE] = [0; SEED_SIZE];

    for byte in password {
        accumulator[0] = accumulator[0].rotate_left((1) as u32) ^ byte;
    }

    for i in 1..SEED_SIZE {
        accumulator[i] = accumulator[i - 1];
        for byte in password {
            accumulator[i] = accumulator[i].rotate_left((i + 1) as u32) ^ byte;
        }
    }

    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_seed() {
        let seed = derive_seed(b"testpassword");
        assert_ne!(seed[0], seed[8]);
        assert_ne!(seed[16], seed[8]);
    }
}