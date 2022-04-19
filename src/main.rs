use crypto_forum::{custom_types::*, *};

#[path = "CLI/user_keypair.rs"]
mod user_keypair;
#[path = "CLI/write.rs"]
mod write;

fn main() {
	let (files, arguments) = get_arguments();
	for messages_file in &files {
		process_file(messages_file, &arguments)
	}
}

fn get_arguments() -> (Vec<String>, Vec<Argument>) {
	let arguments_as_string: Vec<String> = std::env::args().skip(1).collect();
	if arguments_as_string.len() < 1 {
		println!(
			"{program_name}: missing operand",
			program_name = env!("CARGO_PKG_NAME")
		);
		std::process::exit(1)
	}
	let mut files = Vec::<String>::new();
	let mut arguments = Vec::<Argument>::new();
	for arg in arguments_as_string {
		if arg.starts_with("-") {
			arguments.push(parse_dash_argument(arg))
		} else {
			// If there is no "-" at the start of the argument, it's a file that's being passed
			files.push(arg);
		}
	}
	(files, arguments)
}

fn parse_dash_argument(arg: String) -> Argument {
	match arg.as_str() {
		"-i" => return Argument::Interactive,
		"-m" => return Argument::MachineOutput,
		"--version" => {
			println!(
				"{program_name} {program_ver}",
				program_name = env!("CARGO_PKG_NAME"),
				program_ver = env!("CARGO_PKG_VERSION")
			);
			println!("Copyright (C) 2022 Alexander Gorichev\nLicense GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nWritten by Alexander Gorichev.");
			std::process::exit(0)
		}
		_ => {
			println!(
				"{program_name}: invalid option -- '{argument}'",
				program_name = env!("CARGO_PKG_NAME"),
				argument = arg
			);
			std::process::exit(1)
		}
	}
}

fn process_file(messages_file: &String, arguments: &Vec<Argument>) {
	println!("File: {}", messages_file);
	// Read file & handle errors
	let (file_slice, parser) = match read::read_file_data(messages_file) {
		Ok(i) => i,
		Err(Error::StdIo(err)) => match err.kind() {
			std::io::ErrorKind::NotFound => write::make_file(messages_file).unwrap(),
			_ => std::panic!("error"),
		},
		_ => std::panic!("error"),
	};

	let messages = read_serde::get_messages(&file_slice, &parser).unwrap();

	let output_for_machines = arguments.contains(&Argument::MachineOutput);
	if output_for_machines {
		output_for_machine(&messages)
	} else {
		output_for_human(&messages)
	}

	if arguments.contains(&Argument::Interactive) {
		interactive_session(messages_file, parser, messages);
	}
}

fn interactive_session(messages_file: &str, parser: SerdeParser, messages: Vec<Message>) {
	let keypair = user_keypair::login("accounts/").unwrap();
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
		println!(
			"Replying to message with hash: {}",
			bytes_to_hex(&i.prev_hash)
		);
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
	let mut hex_string = String::new();
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
		hex_string.push(hex_from_digit(i / 16));
		hex_string.push(hex_from_digit(i % 16));
	}
	hex_string
}
