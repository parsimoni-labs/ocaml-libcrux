use libcrux_ml_dsa::{
    ml_dsa_65::{self, MLDSA65Signature, MLDSA65SigningKey, MLDSA65VerificationKey},
    KEY_GENERATION_RANDOMNESS_SIZE, SIGNING_RANDOMNESS_SIZE,
};
use rand::RngCore;

/// Error cases
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSigningKey,
    InvalidVerificationKey,
    Signing,
    InvalidSignature,
}

/// Generate a signature key pair.
///
/// * signing_key length = 4032
/// * verification_key length = 1952
#[ocaml::func]
#[ocaml::sig("bytes -> bytes -> unit")] 
pub fn key_gen(signing_key: &mut [u8], verification_key: &mut [u8]) {
    let mut randomness = [0u8; KEY_GENERATION_RANDOMNESS_SIZE];
    rand::rng().fill_bytes(&mut randomness);

    let key_pair = ml_dsa_65::generate_key_pair(randomness);

    signing_key.copy_from_slice(key_pair.signing_key.as_slice());
    verification_key.copy_from_slice(key_pair.verification_key.as_slice());
}

/// Sign `payload` with `signing_key`.
/// * signing_key_len = 4032
/// * signature length = 3309
#[ocaml::func]
#[ocaml::sig("bytes -> bytes -> bytes -> bool")] 
pub fn sign(payload: &[u8], signing_key: &[u8], signature: &mut [u8]) -> bool {
    let signing_key = match signing_key.try_into() {
        Ok(key) => key,
        Err(_) => return false,
    };
    let signing_key = MLDSA65SigningKey::new(signing_key);

    let mut randomness = [0u8; SIGNING_RANDOMNESS_SIZE];
    rand::rng().fill_bytes(&mut randomness);

    match ml_dsa_65::sign(&signing_key, payload, &[], randomness).map_err(|_| Error::Signing) {
        Ok(sig) => {
            signature.copy_from_slice(sig.as_ref());
            true
        }
        Err(_) => false,
    }
}


/// Verify the `signature` on the `payload`, using the `verification_key`.
#[ocaml::func]
#[ocaml::sig("bytes -> bytes -> bytes -> bool")] 
pub fn verify(payload: &[u8], signature: &[u8], verification_key: &[u8]) -> bool {
    let verification_key = match verification_key.try_into() {
        Ok(key) => key,
        Err(_) => return false,
    };
    let verification_key = MLDSA65VerificationKey::new(verification_key);

    let signature = match signature.try_into() {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let signature = MLDSA65Signature::new(signature);

    ml_dsa_65::verify(&verification_key, payload, &[], &signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_test() {
        let (mut signing_key, mut verification_key) = ([0u8; 4032], [0u8; 1952]);
        let mut signature = [0u8; 3309];
        key_gen(&mut signing_key, &mut verification_key);
        assert!(sign(b"hello world", &signing_key, &mut signature));
        assert!(verify(b"hello world", &signature, &verification_key));
    }
}
