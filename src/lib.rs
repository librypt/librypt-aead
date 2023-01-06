
/// Cryptographic nonce.
pub type Nonce<const SIZE: usize> = [u8; SIZE];

/// To use with AEAD symmetric algorithms.
pub trait Aead<const NONCE_SIZE: usize> {
    fn encrypt(
        &mut self, 
        nonce: &Nonce<NONCE_SIZE>,
        paintext: &[u8],
    ) -> Box<[u8]>;

    fn decrypt(
        &mut self,
        nonce: &Nonce<NONCE_SIZE>,
        ciphertext: &[u8],
    ) -> Box<[u8]>;
}