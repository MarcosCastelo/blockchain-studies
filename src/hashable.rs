pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
       crypto_hash::digest(crypt_hash::Algorith::SHA256, &self.bytes()) 
    }
