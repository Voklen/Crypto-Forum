#[path = "CLI/user_keypair.rs"]
mod user_keypair;
#[path = "CLI/write.rs"]
mod write;

#[path = "backend/read_serde.rs"]
mod read_serde;
#[path = "backend/general_functions.rs"]
mod useful_funcs;
#[path = "backend/write_serde.rs"]
mod write_serde;

#[path = "backend/read.rs"]
mod read;
#[path = "backend/encrypt_decrypt.rs"]
mod encrypt_decrypt;

#[cfg(test)]
#[path ="tests/unit_tests/mod.rs"]
mod unit;

#[derive(Debug)]
pub enum Error {
	StdIo(std::io::Error),
	Encryption(chacha20poly1305::aead::Error),
	SmileError(serde_smile::Error),
	JsonError(serde_json::Error),
	InvalidFileData(String),
	SignatureError(ed25519_dalek::SignatureError)
}

#[derive(Debug)]
pub struct Message {
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
	pub hash: [u8; 64],
}

#[derive(Debug)]
pub struct SignatureMessage {
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

impl SignatureMessage {
	fn get_hash(&self) -> [u8; 64] {
		let mut collection_vector = Vec::<u8>::new();
		collection_vector.extend_from_slice(&self.prev_hash);
		collection_vector.extend_from_slice(&self.public_key.to_bytes());
		collection_vector.extend_from_slice(self.message.as_bytes());
		collection_vector.extend_from_slice(&self.signature.to_bytes());

		useful_funcs::hash(&collection_vector)
	}
}

#[derive(Debug)]
pub enum SerdeParser {
	Json,
	Smile,
}

#[derive(PartialEq)]
pub enum Argument {
	Interactive,
	MachineOutput,
}

fn main() {
	let (files, arguments) = get_arguments();
	for messages_file in &files {
		println!("File: {}", messages_file);
		// Read & parse data from file
		let (file_slice, parser) = match read::read_file_data(messages_file) {
			Ok(i) => i,
			Err(Error::StdIo(ref err)) if err.kind() == std::io::ErrorKind::NotFound => {
				write::make_file(messages_file).unwrap()
			}
			_ => std::panic!("error"),
		};
		let messages = read_serde::get_messages(&file_slice, &parser).unwrap();

		// Set argument variables
		let output_for_machines = arguments.contains(&Argument::MachineOutput);

		// Run interactively and exit...
		if arguments.contains(&Argument::Interactive) {
			interactive_session(messages_file, parser, messages, output_for_machines);
			continue
		}

		// ...Otherwise, display the messages
		if output_for_machines {
			output_for_machine(&messages)
		} else {
			output_for_human(&messages)
		}
	}
}

fn get_arguments() -> (Vec<String>, Vec<Argument>) {
	let args: Vec<String> = std::env::args().skip(1).collect();
	if args.len() < 1 {
		println!(
			"{program_name}: missing operand",
			program_name = env!("CARGO_PKG_NAME")
		);
		std::process::exit(1)
	}
	let mut files = Vec::<String>::new();
	let mut arguments = Vec::<Argument>::new();
	for arg in args {
		if &arg[..1] == "-" {
			arguments.push(process_dash_argument(arg))
		} else {
			// If there is no "-" at the start of the argument, it's a file that's being passed
			files.push(arg);
		}
	}
	(files, arguments)
}

fn process_dash_argument(arg: String) -> Argument {
	match arg.as_str() {
		"-i" => return Argument::Interactive,
		"-m" => return Argument::MachineOutput,
		"--version" => {
			println!("{program_name} {program_ver}",
				program_name = env!("CARGO_PKG_NAME"),
				program_ver = env!("CARGO_PKG_VERSION")
			);
			println!("Copyright (C) 2022 Alexander Gorichev\nLicense GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Alexander Gorichev.");
			std::process::exit(0)
		}
		_ => {
			println!("{program_name}: invalid option -- '{argument}'",
				program_name = env!("CARGO_PKG_NAME"),
				argument = arg
			);
			std::process::exit(1)
		}
	}
}

fn interactive_session(messages_file: &str, parser: SerdeParser, messages: Vec<Message>, output_for_machines: bool) {
	if output_for_machines {
		output_for_machine(&messages)
	} else {
		output_for_human(&messages)
	}

	let keypair = user_keypair::login().unwrap();
	let last_hash = match messages.last() {
		Some(i) => i.hash,
		None => [0; 64],
	};
	write::interactive_write(messages_file, &parser, keypair, last_hash);
}

fn output_for_human(messages: &Vec<Message>) {
	for i in messages {
		println!("--------");
		if !i.signed {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		println!("Public key: {}", bytes_to_hex(i.public_key.as_bytes()));
		println!("Replying to message with hash: {}", bytes_to_hex(&i.prev_hash));
		println!("Message: \n{}", i.message);
		println!("Hash: {}", bytes_to_hex(&i.hash));
		println!("--------")
	}
}

fn output_for_machine(messages: &Vec<Message>) {
	for i in messages {
		// Print the message at the end because the message could contain spaces, keywords, and who know what (and has an unknown size) which would make anything after it harder to parse
		println!(
			"Public_key {public_key} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {message}",
			public_key = bytes_to_hex(i.public_key.as_bytes()),
			prev_hash = bytes_to_hex(&i.prev_hash),
			hash = bytes_to_hex(&i.hash),
			signed = i.signed,
			message = i.message,
		);
	}
}

fn bytes_to_hex(bytes: &[u8]) -> String {
	let mut out = String::new();
    for i in bytes {
        fn hex_from_digit(num: u8) -> char {
            if num < 10 {
                (b'0' + num) as char
            } else {
                (b'A' + num - 10) as char
            }
        }
		/*
		Amazing, goes from 0 -> 00:
		println!("1: {}", hex_from_digit(0 / 16));
		println!("2: {}", hex_from_digit(0 % 16));
		all the way to the u8 limit of 255 -> FF (Just like colours! I'm getting way to exited about this…):
		println!("1: {}", hex_from_digit(255 / 16));
		println!("2: {}", hex_from_digit(255 % 16));
		*/
        out.push(hex_from_digit(i / 16));
        out.push(hex_from_digit(i % 16));
    }
	out
}
