/// To use with AEAD symmetric algorithms.
pub trait Aead {
    /// Cryptographic nonce.
    type Nonce;

    fn encrypt(&mut self, key: &Self::Nonce, paintext: &[u8]) -> Box<[u8]>;

    fn decrypt(&mut self, key: &Self::Nonce, ciphertext: &[u8]) -> Box<[u8]>;
}
