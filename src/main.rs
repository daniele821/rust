use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

fn main() {
    let plaintext = "vivekshuk.la".to_string();

    let key_str = "thiskeystrmustbe32charlongtowork".to_string();

    let encrypted_data = encrypt(&key_str, &plaintext);

    println!("encrypted_data: {:?}", encrypted_data.clone());

    let original = decrypt(&key_str, &encrypted_data);

    println!("original: {original:?}");
}

fn encrypt(key_str: &str, plaintext: &str) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");

    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    hex::encode(encrypted_data)
}

fn decrypt(key_str: &str, encrypted_data: &str) -> String {
    let encrypted_data = hex::decode(encrypted_data).expect("failed to decode hex string into vec");

    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());

    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);

    let cipher = Aes256Gcm::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");

    String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn fails() {
        assert_eq!(std::env::var("HOME"), Ok(String::from("/home/daniele")));
    }
}
