extern crate ed25519_dalek;

// use ed25519_dalek::SecretKey;
// use ed25519_dalek::PublicKey;
// use ed25519_dalek::SECRET_KEY_LENGTH;
// use ed25519_dalek::Keypair;
// use ed25519_dalek::Signature;
// use ed25519_dalek::Signer;
// use ed25519_dalek::Verifier;
use ed25519_dalek::*;

fn main() {
	let secret_seed: &[u8; 64] = b"833fe62409237b9d62ec77587520911e9a759cec1d19755b7da901b96dca3d42";
	let message: &[u8] = b"Yes, send all my money to Voklen";

	let secret: SecretKey = SecretKey::from_bytes(&secret_seed[..SECRET_KEY_LENGTH]).unwrap();
	let public: PublicKey = PublicKey::from(&secret);
	let keypair: Keypair = Keypair {
		secret: secret,
		public: public,
	};

	let signature: Signature = keypair.sign(message);

	if public.verify(message, &signature).is_ok() {
		println!("YASS")
	} else {
		println!("aw")
	}
}