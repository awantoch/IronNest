/// TP-Link protocol encryption/decryption functionality
///
/// TP-Link devices use a simple XOR encryption scheme with a key that changes
/// after each byte. The initial key is 0xAB.
const KEY: u8 = 0xAB;

/// Encrypts data with a 4-byte length header for TCP communication
pub fn encrypt_with_header(input: &[u8]) -> Vec<u8> {
    (input.len() as u32)
        .to_be_bytes()
        .into_iter()
        .chain(input.iter().scan(KEY, |key, byte| {
            let result = *byte ^ *key;
            *key = result;
            Some(result)
        }))
        .collect()
}

/// Decrypts data received from UDP broadcasts (no header)
pub fn decrypt(input: &[u8]) -> Vec<u8> {
    let mut buf = input.to_vec();
    let mut key = KEY;
    for item in &mut buf {
        let next_key = *item;
        *item ^= key;
        key = next_key;
    }
    buf
}

/// Encrypts data for UDP broadcasts (no header)
pub fn encrypt(input: &[u8]) -> Vec<u8> {
    let mut buf = input.to_vec();
    let mut key = KEY;
    for byte in &mut buf {
        *byte ^= key;
        key = *byte;
    }
    buf
}

/// Decrypts data with a 4-byte length header from TCP communication
pub fn decrypt_with_header(input: &[u8]) -> Vec<u8> {
    let len = u32::from_be_bytes(input[0..4].try_into().unwrap());
    let mut msg = decrypt(&input[4..]);
    msg.truncate(len as usize);
    msg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = b"Hello, TP-Link!";
        let encrypted = encrypt(original);
        let decrypted = decrypt(&encrypted);

        assert_eq!(original, decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_decrypt_with_header_roundtrip() {
        let original = b"Hello, TP-Link with header!";
        let encrypted = encrypt_with_header(original);
        let decrypted = decrypt_with_header(&encrypted);

        assert_eq!(original, decrypted.as_slice());
    }
}
