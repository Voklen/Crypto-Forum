use crate::{write_serde, SerdeParser, SignatureMessage};
use ed25519_dalek::*;

pub fn interactive_write(file: &str, file_slice: &Vec<u8>, parser: &SerdeParser, keypair: Keypair) {
	let write_data = Vec::<SignatureMessage>::new();

	// THIS BREAKS IF THEIR KEY SEED IS ALL 0'S
	let bad_secret: SecretKey = SecretKey::from_bytes(&[0; SECRET_KEY_LENGTH]).unwrap();
	let bad_public: PublicKey = PublicKey::from(&bad_secret);
	let bad_keypair = Keypair {
		secret: bad_secret,
		public: bad_public,
	};

	let messages = get_messages_from_user(&keypair, write_data, bad_keypair);
	match write_serde::write_to_smile(file, file_slice, &parser, messages) {
		Ok(()) => {}
		Err(_) => {
			println!("Failed to write to file");
			interactive_write(file, file_slice, parser, keypair)
		}
	}
}

fn get_messages_from_user(
	keypair: &Keypair,
	mut write_data: Vec<SignatureMessage>,
	bad_keypair: Keypair,
) -> Vec<SignatureMessage> {
	println!("Please enter desired message");
	let message: String = text_io::try_read!("{}\n").unwrap();
	println!("Would you like to properly sign it? (true/false)");
	let signature: Signature = 
		if text_io::try_read!("{}\n").unwrap() {
			keypair.sign(message.as_bytes())
		} else {
			bad_keypair.sign(message.as_bytes())
		};

	let new_element = SignatureMessage {
		public_key: keypair.public,
		message,
		signature,
	};
	write_data.push(new_element);

	println!("Would you like to enter another message? (true/false)");
	let res: bool = text_io::try_read!("{}\n").unwrap();
	if !res {
		return write_data;
	}
	get_messages_from_user(keypair, write_data, bad_keypair)
}
