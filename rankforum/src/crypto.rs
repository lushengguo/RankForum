use ring::signature::{self, UnparsedPublicKey};

pub fn verify_signature(pubkey: &[u8], signed_data: &[u8], expect_origin_data: &[u8]) -> bool {
    let public_key = UnparsedPublicKey::new(&signature::ED25519, &pubkey);
    if public_key.verify(expect_origin_data, signed_data).is_ok() {
        true
    } else {
        false
    }
}
