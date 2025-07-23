use rand::{RngCore, rng};

pub fn generate_token(byte_length: usize) -> String {
    let mut bytes = vec![0u8; byte_length];
    rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
