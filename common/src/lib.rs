use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub const BLOCK_SIZE: usize = 16;
pub const KEY_SIZE: usize = 16;

pub fn encrypt_aes128(data: &[u8], key_bytes: &[u8; 16]) -> Vec<u8> {
    let key = GenericArray::from(*key_bytes);
    let cipher = Aes128::new(&key);
    
    // PKCS7-like padding
    let padding_size = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let mut padded_data = data.to_vec();
    padded_data.extend(vec![padding_size as u8; padding_size]);
    
    let mut encrypted = Vec::with_capacity(padded_data.len());
    for block in padded_data.chunks(BLOCK_SIZE) {
        let mut block_array = GenericArray::clone_from_slice(block);
        cipher.encrypt_block(&mut block_array);
        encrypted.extend_from_slice(&block_array);
    }
    encrypted
}

pub fn decrypt_aes128(encrypted_data: &[u8], key_bytes: &[u8; 16]) -> Result<Vec<u8>, &'static str> {
    if encrypted_data.len() % BLOCK_SIZE != 0 {
        return Err("Invalid encrypted data length");
    }

    let key = GenericArray::from(*key_bytes);
    let cipher = Aes128::new(&key);
    
    let mut decrypted = Vec::with_capacity(encrypted_data.len());
    for block in encrypted_data.chunks(BLOCK_SIZE) {
        let mut block_array = GenericArray::clone_from_slice(block);
        cipher.decrypt_block(&mut block_array);
        decrypted.extend_from_slice(&block_array);
    }
    
    // Remove padding
    if let Some(&padding_size) = decrypted.last() {
        let pad_len = padding_size as usize;
        if pad_len > 0 && pad_len <= BLOCK_SIZE && pad_len <= decrypted.len() {
            decrypted.truncate(decrypted.len() - pad_len);
            return Ok(decrypted);
        }
    }
    
    Err("Invalid padding detected")
}

/// Simple XOR obfuscation/deobfuscation for strings and small buffers
pub fn xor_data(data: &mut [u8], key: u8) {
    for byte in data.iter_mut() {
        *byte ^= key;
    }
}

/// Simple DJB2 hash for string obfuscation and API resolving.
pub fn djb2_hash(s: &str) -> u32 {
    let mut hash: u32 = 5381;
    for c in s.bytes() {
        hash = (hash << 5).wrapping_add(hash).wrapping_add(c as u32);
    }
    hash
}
