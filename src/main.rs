use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce}; // Or use Aes128Gcm depending on key size
use std::fs;

fn decrypt_file(cipher_file: &str, key_file: &str) -> String {
    // Read key
    let key = fs::read(key_file).unwrap();
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap(); // Aes128Gcm for 16-byte keys

    // Read encrypted file
    let data = fs::read(cipher_file).unwrap();

    // Extract nonce and ciphertext
    let nonce_size = 12; // AES-GCM standard nonce size
    let (nonce_bytes, ciphertext) = data.split_at(nonce_size);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();

    // Convert plaintext bytes to string
    String::from_utf8(plaintext).unwrap()
}

fn main() {
    let cipher_file = "/personal/repos/daniele821/track-payments/data/payments.json.cipher";
    let key_file = "/personal/repos/daniele821/track-payments/data/.cipher_key";

    let decrypted = decrypt_file(cipher_file, key_file);
    println!("{decrypted}");
}
