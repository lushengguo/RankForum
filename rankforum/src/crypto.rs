use base64::prelude::*;
use ring::rand::SystemRandom;
use ring::signature::{self, Ed25519KeyPair, KeyPair, Signature, UnparsedPublicKey};
use rouille::*;
use serde_json;

pub fn verify_signature_of_http_request(request: &Request) -> Result<(), &'static str> {
    let body = input::plain_text_body(&request).unwrap();
    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let pubkey = match json_body.get("pubkey") {
        Some(pubkey) => pubkey.as_str().unwrap(),
        None => return Err("pubkey field is needed in http body"),
    };
    let signed_pubkey = match json_body.get("signed_pubkey") {
        Some(signed_pubkey) => signed_pubkey.as_str().unwrap(),
        None => return Err("signed_pubkey field is needed in http body"),
    };

    let pubkey_bytes = BASE64_STANDARD.decode(pubkey).unwrap();
    let signed_pubkey_bytes = BASE64_STANDARD.decode(signed_pubkey).unwrap();

    let public_key = UnparsedPublicKey::new(&signature::ED25519, &pubkey_bytes);
    let message = pubkey_bytes.as_slice();

    if public_key.verify(message, &signed_pubkey_bytes).is_ok() {
        Ok(())
    } else {
        Err("signature verification failed")
    }
}
