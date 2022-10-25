use crypto_forum::{custom_types::*, hex::*, *};

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
	// Skip the first argument because it's just the executable path
	let arguments_as_strings: Vec<String> = std::env::args().skip(1).collect();

	if arguments_as_strings.is_empty() {
		println!(
			"{program_name}: missing operand",
			program_name = env!("CARGO_PKG_NAME")
		);
		std::process::exit(1)
	}

	let mut files = Vec::<String>::new();
	let mut arguments = Vec::<Argument>::new();
	for arg in arguments_as_strings {
		if arg.starts_with('-') {
			arguments.push(parse_dash_argument(&arg))
		} else {
			// If there is no "-" at the start of the argument, it's a file that's being passed
			files.push(arg);
		}
	}
	(files, arguments)
}

fn parse_dash_argument(arg: &str) -> Argument {
	match arg {
		"-i" => Argument::Interactive,
		"-m" => Argument::MachineOutput,
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

fn process_file(messages_file: &str, arguments: &[Argument]) {
	println!("File: {}", messages_file);
	let file_slice = read_file(messages_file);
	let parser = read::file_type(&file_slice).unwrap_or_else(|| {
		println!("Unknown file type");
		std::process::exit(1)
	});
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

fn read_file(messages_file: &str) -> Vec<u8> {
	std::fs::read(messages_file).unwrap_or_else(|err| match err.kind() {
		std::io::ErrorKind::NotFound => write::make_file(messages_file),
		_ => panic!(),
	})
}

fn interactive_session(messages_file: &str, parser: SerdeParser, messages: Vec<Message>) {
	let keypair = user_keypair::login("reference/accounts/").unwrap();
	let last_hash = match messages.last() {
		Some(i) => i.get_hash(),
		None => [0; 64],
	};
	write::interactive_write(messages_file, &parser, keypair, last_hash);
}

fn output_for_human(messages: &Vec<Message>) {
	for i in messages {
		println!("--------");
		if !i.is_signed() {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		println!("Public key: {}", bytes_to_hex(i.public_key.as_bytes()));
		println!(
			"Replying to message with hash: {}",
			bytes_to_hex(&i.prev_hash)
		);
		println!("Message: \n{}", i.message);
		println!("Hash: {}", bytes_to_hex(&i.get_hash()));
		println!("--------")
	}
}

fn output_for_machine(messages: &Vec<Message>) {
	for i in messages {
		// Print `message` at the end because it could contain spaces, keywords, and who-knows-what (and has an unknown size)
		// Which would make it hard to know when `message` ends meaning anything after it on the same line is harder to parse
		println!(
			"Public_key {public_key} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {message}",
			public_key = bytes_to_hex(i.public_key.as_bytes()),
			prev_hash = bytes_to_hex(&i.prev_hash),
			hash = bytes_to_hex(&i.get_hash()),
			signed = i.is_signed(),
			message = i.message,
		);
	}
}

pub fn input(prompt: &str) -> String {
	println!("{}", prompt);

	let mut input_string = String::new();
	let read_result = std::io::stdin().read_line(&mut input_string);

	if read_result.is_err() {
		println!("Could not read the input, try again.");
		return input(prompt);
	};
	input_string.trim().into()
}

pub fn ask_for_bool(prompt: &str) -> bool {
	let prompt = &format!("{prompt} (true/false)");
	let input = input(prompt);

	match input.trim() {
		"true" => true,
		"false" => false,
		_ => {
			println!("Please only enter true or false");
			ask_for_bool(prompt)
		}
	}
}
