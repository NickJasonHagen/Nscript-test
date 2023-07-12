use crypto::aessafe::AesSafe256Decryptor;
use crypto::aessafe::AesSafe256Encryptor;
use crypto::blockmodes::CbcDecryptor;
use crypto::blockmodes::CbcEncryptor;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::digest::Digest;
use crypto::pbkdf2::pbkdf2;
use crypto::sha2::Sha256;
use rand::RngCore;

const SALT_SIZE: usize = 16;
const KEY_SIZE: usize = 32;
const IV_SIZE: usize = 16;

// Function to encrypt a string
pub fn encrypt_string(input: &str, passphrase: &str) -> String {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt);

    let mut encryptor = CbcEncryptor::new(
        AesSafe256Encryptor::new(&key),
        &vec![0u8; IV_SIZE],
    );
    let mut encrypted_data = Vec::new();
    encryptor.encrypt(input.as_bytes(), &mut encrypted_data, true).unwrap();

    let mut encrypted = Vec::new();
    encrypted.extend_from_slice(&salt);
    encrypted.extend_from_slice(&encrypted_data);

    base64::encode(&encrypted)
}

// Function to generate a random salt
fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; SALT_SIZE];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

// Function to derive a key from the passphrase and salt
fn derive_key(passphrase: &str, salt: &[u8]) -> Vec<u8> {
    let mut key = vec![0u8; KEY_SIZE];
    pbkdf2::<Hmac<Sha256>>(
        passphrase.as_bytes(),
        salt,
        100_000,
        &mut key,
    );
    key
}

// Function to decrypt a string
pub fn decrypt_string(input: &str, passphrase: &str) -> String {
    let decoded = base64::decode(input).unwrap();
    let salt = &decoded[..SALT_SIZE];
    let ciphertext = &decoded[SALT_SIZE..];

    let key = derive_key(passphrase, salt);

    let mut decryptor = CbcDecryptor::new(
        AesSafe256Decryptor::new(&key),
        &vec![0u8; IV_SIZE],
    );
    let mut decrypted_data = Vec::new();
    decryptor.decrypt(ciphertext, &mut decrypted_data).unwrap();

    String::from_utf8_lossy(&decrypted_data).to_string()
}

fn main() {
    let input_string = "Hello, World!";
    let passphrase = "mysecretpassword";

    // Encrypt the string
    let encrypted_string = encrypt_string(input_string, passphrase);
    println!("Encrypted: {}", encrypted_string);

    // Decrypt the string
    let decrypted_string = decrypt_string(&encrypted_string, passphrase);
    println!("Decrypted: {}", decrypted_string);
}
