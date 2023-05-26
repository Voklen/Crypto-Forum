use crypto_forum::{custom_types::*, *};

#[path = "cli/user_keypair.rs"]
mod user_keypair;
#[path = "cli/interactive_write.rs"]
mod write_cli;

fn main() {
	let (links, arguments) = get_arguments();
	if arguments.contains(&Argument::Create) {
		let created_link = write_cli::new_repo();
		process_file(&created_link, &arguments);
		return;
	}
	for messages_file in &links {
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
		"-c" => Argument::Create,
		"-v" => print_version_info(),

		"--interactive" => Argument::Interactive,
		"--machine-output" => Argument::MachineOutput,
		"--create" => Argument::Create,
		"--version" => print_version_info(),

		_ => unknown_arg(arg),
	}
}

fn print_version_info() -> Argument {
	println!(
		"{program_name} {program_ver}",
		program_name = env!("CARGO_PKG_NAME"),
		program_ver = env!("CARGO_PKG_VERSION")
	);
	println!(
		"
Copyright (C) 2022 Alexander Gorichev
License GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Alexander Gorichev.
Send all issues to <https://github.com/Voklen/Crypto-Forum/issues> or <Alex.Gorichev@protonmail.com>
I hope you enjoy :)"
	);
	std::process::exit(0)
}

fn unknown_arg(arg: &str) -> Argument {
	println!(
		"{program_name}: invalid option -- '{argument}'",
		program_name = env!("CARGO_PKG_NAME"),
		argument = arg
	);
	std::process::exit(1)
}

fn process_file(link: &str, arguments: &[Argument]) {
	println!("File: {link}");
	let messages = read::get_messages(link).unwrap();

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
	write_cli::interactive_write(messages_file, keypair, last_hash);
}

fn output_for_human(messages: &Vec<Message>) {
	for message in messages {
		let username = message.get_username();
		let prev_hash = message.prev_hash_string();
		let hash = message.hash_string();
		let body = &message.body;
		println!("--------");
		if !message.is_signed() {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		match username {
			Some(name) => println!("Public key: {name}"),
			None => println!("Public key: {}", message.public_key_string()),
		}
		println!("Replying to message with hash: {prev_hash}");
		println!("Message: \n{body}");
		println!("Hash: {hash}");
		println!("--------")
	}
}

fn output_for_machine(messages: &Vec<Message>) {
	for message in messages {
		let public_key = message.get_username();
		let prev_hash = message.prev_hash_string();
		let hash = message.hash_string();
		let signed = message.is_signed();
		let body = &message.body;
		// Print `body` at the end because it could contain spaces, keywords, and who-knows-what (and has an unknown size)
		// Which would make it hard to know when `body` ends meaning anything after it on the same line is harder to parse
		match public_key {
			Some(username) => println!(
				"Username {username} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {body}"
			),
			None => println!(
				"Public_key {public} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {body}",
				public = message.public_key_string()
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
