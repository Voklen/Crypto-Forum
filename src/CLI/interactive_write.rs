use crate::{ask_for_bool, input, write_serde, Message};
use ed25519_dalek::*;

pub fn interactive_write(file: &str, keypair: Keypair, last_hash: [u8; 64]) {
	let write_data = Vec::<Message>::new();

	// THIS BREAKS IF THEIR KEY SEED IS ALL 0'S
	let bad_secret: SecretKey = SecretKey::from_bytes(&[0; SECRET_KEY_LENGTH]).unwrap();
	let bad_public: PublicKey = PublicKey::from(&bad_secret);
	let bad_keypair = Keypair {
		secret: bad_secret,
		public: bad_public,
	};

	let messages = get_messages_from_user(&keypair, write_data, last_hash, &bad_keypair);
	let write_result = write_serde::write_messages(file, messages);
	if write_result.is_err() {
		println!("Failed to write to file");
		interactive_write(file, keypair, last_hash)
	};
}

fn get_messages_from_user(
	keypair: &Keypair,
	mut write_data: Vec<Message>,
	prev_hash: [u8; 64],
	bad_keypair: &Keypair,
) -> Vec<Message> {
	let message = input("Please enter desired message");
	let to_sign = &[message.as_bytes(), &prev_hash].concat();

	let signature = if ask_for_bool("Would you like to properly sign it?") {
		keypair.sign(to_sign)
	} else {
		bad_keypair.sign(to_sign)
	};

	let new_element = Message {
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

pub fn new_repo(links: &Vec<String>) {
	if links.is_empty() {
		eprintln!("Please provide repo name with -c");
		std::process::exit(0);
	}
	if links.len() != 1 {
		eprintln!("Please only provide one repo name with -c");
		std::process::exit(0);
	}
	match write_serde::new_ipns(&links[0]) {
		Ok(ipns_link) => println!("Repo made at link: {ipns_link}"),
		Err(error) => {
			eprintln!("Failed to create repo with error: {:?}", error);
			std::process::exit(0);
		}
	};
}
