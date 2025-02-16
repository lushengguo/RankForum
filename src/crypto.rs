use ring::signature::{self, UnparsedPublicKey};

pub fn verify_signature(pubkey: &[u8], signed_data: &[u8], expect_origin_data: &[u8]) -> bool {
    let public_key = UnparsedPublicKey::new(&signature::ED25519, &pubkey);
    public_key.verify(expect_origin_data, signed_data).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::rand;
    use ring::signature::{self, KeyPair};

    fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
        let rng = rand::SystemRandom::new();
        let pkcs8 = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        (key_pair.public_key().as_ref().to_vec(), pkcs8.as_ref().to_vec()) 
    }

    #[test]
    fn test_verify_signature() {
        let (pubkey, privkey) = generate_keypair();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(&privkey).unwrap();

        let data = b"hello, world";
        let signature = key_pair.sign(data);

        assert!(verify_signature(&pubkey, signature.as_ref(), data));
    }

    #[test]
    fn test_verify_signature_with_invalid_pubkey() {
        let (_valid_pubkey, privkey) = generate_keypair();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(&privkey).unwrap();

        let data = b"test data";
        let signature = key_pair.sign(data);

        let invalid_pubkey = vec![0; 32]; 
        assert!(!verify_signature(&invalid_pubkey, signature.as_ref(), data));
    }

    #[test]
    fn test_verify_signature_with_tampered_data() {
        let (pubkey, privkey) = generate_keypair();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(&privkey).unwrap();

        let data = b"original data";
        let signature = key_pair.sign(data);

        let tampered_data = b"modified data";
        assert!(!verify_signature(&pubkey, signature.as_ref(), tampered_data));
    }

    #[test]
    fn test_verify_signature_with_tampered_signature() {
        let (pubkey, privkey) = generate_keypair();
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(&privkey).unwrap();

        let data = b"secure data";
        let mut signature = key_pair.sign(data).as_ref().to_vec();

        
        signature[0] ^= 0xFF;
        assert!(!verify_signature(&pubkey, &signature, data));
    }
}
