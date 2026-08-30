use crate::constants::SEED_SIZE;


/// XORs `data` against `keystream` byte-by-byte. Used for both encryption
/// and decryption, since XOR is its own inverse -> the same operation
/// applied twice with the same keystream recovers the original bytes.
pub fn encrypt(data: &[u8], keystream: Vec<u8>) -> Vec<u8> {
    data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect()
}

/// Identical operation to `encrypt` -> XOR is symmetric, so decryption
/// is just encryption applied again with the same keystream.
pub fn decrypt(encrypted_data: &[u8], keystream: Vec<u8>) -> Vec<u8> {
    encrypted_data.iter().zip(keystream.iter()).map(|pair| pair.0 ^ pair.1).collect()
}

/// Generates `data_length` bytes of keystream from `seed`, starting at
/// `pointer` bytes into the seed's infinitely repeating cycle. Used to
/// keep the keystream in sync across chunked reads/writes, where each
/// chunk continues from where the previous one left off.
pub fn generate_keystream(seed: &[u8; SEED_SIZE], data_length: usize, pointer: usize) -> Vec<u8> {
    // key_bytes.iter().cycle().take(length).copied() -> lazy stages; copied gives ownership from references
    // .collec() -> consuming adaptor

    let keystream: Vec<u8> = seed.iter().cycle().take(pointer + data_length).copied().collect();

    keystream[pointer..].to_vec()
}


// Old logic (had a rotation-mod-8 collision, see: seed[0] == seed[8]):
// for i in 0..SEED_SIZE {
//     for byte in password {
//         accumulator[i] = accumulator[i].rotate_left((i + 1) as u32) ^ byte;
//     }
// }
// But this was causing a issue i.e., every i/8th rotation is same means 1, 9, 17 produces 
// same number after rotation when rotating number is of 8 bit

/// Derives a fixed-size seed from a password using a chained mixing process
/// (rotate + XOR), where each output byte depends on the accumulated state
/// from all previous bytes rather than a fresh computation per position.
pub fn derive_seed(password: &[u8]) -> [u8; SEED_SIZE] {

    let mut accumulator:[u8; SEED_SIZE] = [0; SEED_SIZE];

    for byte in password {
        accumulator[0] = accumulator[0].rotate_left(1_u32) ^ byte;
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

    #[test]
    fn round_trip_test() {
        let data = b"hello world!";

        let password = "hunter";

        let seed = derive_seed(password.as_bytes());

        let keystream = generate_keystream(&seed, data.len(), 0);

        let encrypted_data = encrypt(&data.to_vec(), keystream.clone());

        let decrypted_data = decrypt(&encrypted_data, keystream);

        assert_eq!(decrypted_data, data)
    }

    #[test]
    fn wrong_key_produces_different_output() {
        let data = b"hello world!";

        // encryption
        let password = "hunter";

        let seed = derive_seed(password.as_bytes());

        let keystream = generate_keystream(&seed, data.len(), 0);

        let encrypted_data = encrypt(&data.to_vec(), keystream.clone());

        // decryption
        let password = "xhunter";

        let seed = derive_seed(password.as_bytes());

        let keystream = generate_keystream(&seed, data.len(), 0);

        let decrypted_data = decrypt(&encrypted_data, keystream);

        assert_ne!(decrypted_data, data)    
    }

    #[test]
    fn empty_input_test() {
        let data = b"";

        let password = "hunter";

        let seed = derive_seed(password.as_bytes());

        let keystream = generate_keystream(&seed, data.len(), 0);

        let encrypted_data = encrypt(&data.to_vec(), keystream.clone());

        let decrypted_data = decrypt(&encrypted_data, keystream);

        assert_eq!(decrypted_data, data)
    }
}