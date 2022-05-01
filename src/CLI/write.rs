use crate::{ask_for_bool, input, write_serde, MessageForWriting, SerdeParser};
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

	let messages = get_messages_from_user(&keypair, write_data, last_hash, &bad_keypair);
	let write_result = write_serde::write_messages(file, &parser, messages);
	if write_result.is_err() {
		println!("Failed to write to file");
		interactive_write(file, parser, keypair, last_hash)
	};
}

fn get_messages_from_user(
	keypair: &Keypair,
	mut write_data: Vec<MessageForWriting>,
	prev_hash: [u8; 64],
	bad_keypair: &Keypair,
) -> Vec<MessageForWriting> {
	let message = input("Please enter desired message");
	let to_sign = &[message.as_bytes(), &prev_hash].concat();

	let signature = if ask_for_bool("Would you like to properly sign it?") {
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

	if !ask_for_bool("Would you like to enter another message?") {
		return write_data;
	}
	get_messages_from_user(keypair, write_data, new_hash, bad_keypair)
}

pub fn make_file(file: &str) -> Vec<u8> {
	let should_make_file = ask_for_bool("Would you like to make a file?");
	if !should_make_file {
		println!("No file made");
		std::process::exit(0);
	}

	let parser = ask_for_parser();
	let slice = write_serde::write_messages(file, &parser, Vec::<MessageForWriting>::new());

	match slice {
		Ok(i) => i,
		Err(err) => {
			println!("Could not write to file: {:?}", err);
			make_file(file)
		}
	}
}

// See Decisions.md for explanation
fn ask_for_parser() -> SerdeParser {
	use strum::IntoEnumIterator;
	println!("Possible file types:");
	let result_vec: Vec<SerdeParser> = SerdeParser::iter()
		.enumerate()
		.map(|(index, file_type)| {
			println!("{}) {}", index + 1, file_type);
			file_type
		})
		.collect();

	let input_usize = ask_for_usize();
	match result_vec.into_iter().nth(input_usize + 1) {
		Some(i) => i,
		None => {
			println!("Please pick a number on the list");
			ask_for_parser()
		}
	}
}

fn ask_for_usize() -> usize {
	match input("Select an option (enter the number)").parse() {
		Ok(i) => i,
		Err(_) => {
			println!("Please enter a positive number");
			ask_for_usize()
		}
	}
}
