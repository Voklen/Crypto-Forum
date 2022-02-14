extern crate ed25519_dalek;
extern crate text_io;

mod user_keypair;
mod write;
mod read;

// use ed25519_dalek::SecretKey;
// use ed25519_dalek::PublicKey;
// use ed25519_dalek::SECRET_KEY_LENGTH;
// use ed25519_dalek::Keypair;
// use ed25519_dalek::Signature;
// use ed25519_dalek::Signer;
// use ed25519_dalek::Verifier;
// use ed25519_dalek::Sha512;
use ed25519_dalek::*;
use text_io::try_read;

fn main() {
	let keypair = user_keypair::get_keypair();
	write::interactive_write(keypair);
	let messages = read::main();
}