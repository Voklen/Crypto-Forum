use crate::{write_serde, SerdeParser, MessageForWriting};
use ed25519_dalek::*;

pub fn interactive_write(file: &str, parser: &SerdeParser, keypair: Keypair, last_hash: [u8; 64]) {
	let write_data = Vec::<MessageForWriting>::new();

	// THIS BREAKS IF THEIR KEY SEED IS ALL 0'S
	let bad_secret: SecretKey = SecretKey::from_bytes(&[0; SECRET_KEY_LENGTH]).unwrap();
	let bad_public: PublicKey = PublicKey::from(&bad_secret);
	let bad_keypair = Keypair {
		secret: bad_secret,
		public: bad_public,
	};

	let messages = get_messages_from_user(&keypair, write_data, last_hash, bad_keypair);
	write_serde::write_messages(file, &parser, messages).unwrap_or_else(|_| {
		println!("Failed to write to file");
		interactive_write(file, parser, keypair, last_hash)
	})
}

fn get_messages_from_user(
	keypair: &Keypair,
	mut write_data: Vec<MessageForWriting>,
	prev_hash: [u8; 64],
	bad_keypair: Keypair,
) -> Vec<MessageForWriting> {
	println!("Please enter desired message");
	let message: String = text_io::try_read!("{}\n").unwrap();
	let to_sign = &[message.as_bytes(), &prev_hash].concat();
	
	println!("Would you like to properly sign it? (true/false)");
	let signature: Signature = if text_io::try_read!("{}\n").unwrap() {
		keypair.sign(to_sign)
	} else {
		bad_keypair.sign(to_sign)
	};

	let new_element = MessageForWriting {
		prev_hash,
		public_key: keypair.public,
		message,
		signature,
	};
	let new_hash = new_element.get_hash(); // This line is here so we can get the hash before it's moved into write_data
	write_data.push(new_element);

	println!("Would you like to enter another message? (true/false)");
	let res: bool = text_io::try_read!("{}\n").unwrap();
	if !res {
		return write_data;
	}
	get_messages_from_user(keypair, write_data, new_hash, bad_keypair)
}

pub fn make_file(file: &str) -> Vec<u8> {
	// Get user input
	println!("Would you like to make a file? (true/false)");
	let should_make_file: bool = match text_io::try_read!("{}\n") {
		Ok(i) => i,
		Err(_) => {
			println!("Please type either true or false");
			return make_file(file);
		}
	};

	// Exit if user says so
	if !should_make_file {
		println!("No file made");
		std::process::exit(0);
	}

	//TODO Change empty file based on parser type
	let slice = "[[]]".as_bytes();

	// Write to file
	match std::fs::write(file, slice) {
		Ok(_) => {},
		Err(err) => {
			println!("Could not write to file: {}", err);
			return make_file(file);
		}
	}

	slice.to_vec()
}
