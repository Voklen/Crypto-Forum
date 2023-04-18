use crypto_forum::{custom_types::*, *};

#[path = "CLI/user_keypair.rs"]
mod user_keypair;
#[path = "CLI/interactive_write.rs"]
mod write;

fn main() {
	let (links, arguments) = get_arguments();
	for messages_file in &links {
		process_file(messages_file, &arguments)
	}
}

#[derive(PartialEq)]
pub enum Argument {
	Interactive,
	MachineOutput,
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

	let mut links = Vec::<String>::new();
	let mut arguments = Vec::<Argument>::new();
	for arg in arguments_as_strings {
		if arg.starts_with('-') {
			arguments.push(parse_dash_argument(&arg))
		} else {
			// If there is no "-" at the start of the argument, it's a link that's being passed
			links.push(arg);
		}
	}
	(links, arguments)
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

fn process_file(link: &str, arguments: &[Argument]) {
	println!("File: {}", link);
	let messages = read_serde::get_messages(link).unwrap();

	let output_for_machines = arguments.contains(&Argument::MachineOutput);
	if output_for_machines {
		output_for_machine(&messages)
	} else {
		output_for_human(&messages)
	}

	if arguments.contains(&Argument::Interactive) {
		interactive_session(link, messages);
	}
}

fn interactive_session(messages_file: &str, messages: Vec<Message>) {
	let keypair = user_keypair::login("reference/accounts/").unwrap();
	let last_hash = match messages.last() {
		Some(i) => i.get_hash(),
		None => [0; 64],
	};
	write::interactive_write(messages_file, keypair, last_hash);
}

fn output_for_human(messages: &Vec<Message>) {
	for i in messages {
		let username = i.get_username();
		let prev_hash = i.hex_prev_hash();
		let hash = i.hex_hash();
		let message = &i.message;
		println!("--------");
		if !i.is_signed() {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		match username {
			Some(name) => println!("Public key: {name}"),
			None => println!("Public key: {}", i.hex_public_key()),
		}
		println!("Replying to message with hash: {prev_hash}");
		println!("Message: \n{message}");
		println!("Hash: {hash}");
		println!("--------")
	}
}

fn output_for_machine(messages: &Vec<Message>) {
	for i in messages {
		let public_key = i.get_username();
		let prev_hash = i.hex_prev_hash();
		let hash = i.hex_hash();
		let signed = i.is_signed();
		let message = &i.message;
		// Print `message` at the end because it could contain spaces, keywords, and who-knows-what (and has an unknown size)
		// Which would make it hard to know when `message` ends meaning anything after it on the same line is harder to parse
		match public_key {
			Some(username) => println!(
				"Username {username} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {message}"
			),
			None => println!(
				"Public_key {public} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {message}",
				public = i.hex_public_key()
			),
		}
	}
}

pub fn input(prompt: &str) -> String {
	println!("{prompt}");

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
