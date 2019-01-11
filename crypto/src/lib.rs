/* Handles cryptographic functions.
 * Depending on platform, this may call various libraries or have other dependencies.
 *
 * This uses prime256v1 EC encryption that should come from internal crypto calls. The "application-services"
 * module compiles openssl, however, so might be enough to tie into that. 
 */
use std::collections::HashMap;

pub struct CryptoError;

pub struct Key {
    pub private: Vec<u8>,
    pub public: Vec<u8>,
    pub auth: Vec<u8>,
}

pub trait Cryptography {
    // generate a new key (Use Into:: to dump this as JSON if needed)
    fn generate_key() -> Key;
    // Derive a new key from the passed public key vec
    fn derive(public_key: &Vec<u8>) -> Key;

    // General decrypt function. Calls to decrypt_aesgcm or decrypt_aes128gcm as needed.
    // (sigh, can't use notifier::Notification because of circular dependencies.)
    fn decrypt(
        body: Vec<u8>,
        encoding: &str,
        salt: Option<Vec<u8>>,
        dh: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, CryptoError>;
    // IIUC: objects created on one side of FFI can't be freed on the other side, so we have to use references (or clone)
    fn decrypt_aesgcm(
        content: &Vec<u8>,
        auth_key: &str,
        salt: &Vec<u8>,
        crypto_key: &Vec<u8>,
    ) -> Result<Vec<u8>, CryptoError>;
    fn decrypt_aes128gcm(content: &Vec<u8>, auth_key: &Vec<u8>) -> Result<Vec<u8>, CryptoError>;
}

pub struct Crypto;

impl Cryptography for Crypto {
    fn generate_key() -> Key
    {
        Key {
            private: Vec::new(),
            public: Vec::new(),
            auth: Vec::new()
        }
    }

    fn derive(public_key: &Vec<u8>) -> Key
    {
        Key {
            private: Vec::new(),
            public: Vec::new(),
            auth: Vec::new()
        }
    }

    // General decrypt function. Calls to decrypt_aesgcm or decrypt_aes128gcm as needed.
    // (sigh, can't use notifier::Notification because of circular dependencies.)
    fn decrypt(
        body: Vec<u8>,
        encoding: &str,
        salt: Option<Vec<u8>>,
        dh: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, CryptoError>{
        Err(CryptoError)
    }

    // IIUC: objects created on one side of FFI can't be freed on the other side, so we have to use references (or clone)
    fn decrypt_aesgcm(
        content: &Vec<u8>,
        auth_key: &str,
        salt: &Vec<u8>,
        crypto_key: &Vec<u8>,
    ) -> Result<Vec<u8>, CryptoError>{
        Err(CryptoError)
    }

    fn decrypt_aes128gcm(content: &Vec<u8>, auth_key: &Vec<u8>) -> Result<Vec<u8>, CryptoError>{
        Err(CryptoError)
    }
}
