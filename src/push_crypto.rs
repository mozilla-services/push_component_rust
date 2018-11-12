use std::collections::HashMap;

use futures::Future;

use unknown::*;

use push_service::PushError;

pub struct CryptoError {
    message: String,
    isCryptoError: bool,
    property: String,
    params: Vec<String>,
}

pub struct PushCryptoParams {
    salt: Vec<u8>,
    rs: Vec<u8>,
    senderKey: Vec<u8>,
    ciphertext: Vec<u8>,
}

pub fn getEncryptionKeyParams(encryptKeyField: String) -> Result<PushCryptoParams, CryptoError> {}

// parse the encryption key header
pub fn getEncryptionParams(encryptField: String) -> Result<PushCryptoParams, CryptoError> {}

// parse the encryption header
pub fn getCryptoParamsFromPayload(payload: Vec<u8>) -> Result<PushCryptoParams, CryptoError> {}

// parse encryption vals from the payload
pub fn getCryptoParamsFromHeaders(
    headers: HashMap<String, String>,
) -> Result<PushCryptoParams, CryptoError> {
}

// wrapper to pull crypto info from headers.
pub fn base64URLDecode(value: String) -> Result<Vec<u8>, PushError> {}

pub fn chunkArray(array: Vec<u8>, size: usize) -> Vec<u8> {}
pub fn concatArray(arrays: Vec<Vec<u8>>) -> Vec<u8> {}
pub fn hmac(key: Vec<u8>) -> Vec<u8> {}
pub fn hmac_hash(hmac: Vec<u8>, input: Vec<u8>) -> Vec<u8> {}
pub fn hkdf(salt: Vec<u8>, ikm: Vec<u8>) -> Vec<u8> {}
pub fn hkdf_extract(info: Vec<u8>, len: usize) -> Vec<u8> {}
pub fn generateNonce(base: u64, index: usize) -> Vec<u8> {}
pub fn encodeLength(buffer: Vec<u8>) -> Vec<u8> {}

pub struct Decoder {}

impl Decoder {
    pub fn new(
        privateKey: JSONWebKey,
        publicKey: Vec<u8>,
        authenticationSecret: Vec<u8>,
        cryptoParams: PushCryptoParams,
        cipherText: Vec<u8>,
    ) {
    }
    pub fn decode() -> Result<Vec<u8>, PushError> {}
    pub fn computeSharedSecret() -> Vec<u8> {}
    pub fn decodeChunk(
        slice: u64,
        index: u64,
        nonce: Vec<u8>,
        key: Vec<u8>,
        last: bool,
    ) -> Result<Vec<u8>, PushError> {
    }
    pub fn unpadChunk(chunk: Vec<u8>, last: bool) -> Vec<u8> {}
    pub fn chunkSize() -> u64 {}
}

trait OldSchemeDecoder {
    fn decode() -> Vec<u8>;
    fn unpadChunk(decoded: Vec<u8>) -> Vec<u8>;
    fn chunkSize() -> u64;
    fn padSize() -> u64;
}

trait aes128gcmDecoder {
    fn unpadChunk(decoded: Vec<u8>, last: bool) -> Vec<u8>;
    fn chunkSize() -> u64;
    fn deriveKeyAndNonce(ikm: Vec<u8>) -> Result<Vec<u8>, CryptoError>;
}

trait PushCrypto {
    fn generateAuthenticationSecret() -> Vec<u8>;
    fn validateAppServerKey(key: Vec<u8>) -> Result<Vec<u8>, CryptoError>;
    fn generateKeys() -> Vec<Vec<u8>>;
}

pub fn decrypt(
    privateKey: Vec<u8>,
    publicKey: Vec<u8>,
    authenticationSecret: Vec<u8>,
    headers: HashMap<String, String>,
    payload: Vec<u8>,
) -> Result<Vec<u8>, CryptoError> {
}

pub fn encrypt(
    plaintext: Vec<u8>,
    receiverPublicKey: Vec<u8>,
    receiverAuthSecret: Vec<u8>,
    Options: PushCryptoParams,
) -> (Vec<u8>, String) {
}

trait aes128gcmEncoder {
    fn new(
        plaintext: Vec<u8>,
        receiverPublicKey: Vec<u8>,
        receiverAuthSecret: Vec<u8>,
        senderKeyPair: Vec<u8>,
        salt: Vec<u8>,
        rs: Vec<u8>,
    );
    fn encode();
    fn encrypt(key: Vec<u8>, nonce: Vec<u8>) -> Result<Vec<Vec<u8>>, CryptoError>;
    fn deriveKeyAndNonce(
        sharedSecret: Vec<u8>,
        senderPublicKey: Vec<u8>,
    ) -> Future<Item = Vec<u8>, Error = CryptoError>;
    fn computeSharedSecret(
        receiverPublicKey: Vec<u8>,
        senderPrivateKey: Vec<u8>,
    ) -> Result<Vec<u8>, CryptoError>;
    fn createHeader(key: Vec<u8>) -> Vec<u8>;
}
