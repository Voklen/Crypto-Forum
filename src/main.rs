use crypto_forum::{custom_types::*, *};

#[path = "cli/arguments.rs"]
mod arguments;
#[path = "cli/input.rs"]
mod input;
#[path = "cli/user_keypair.rs"]
mod user_keypair;
#[path = "cli/interactive_write.rs"]
mod write_cli;

fn main() {
	let (links, arguments) = arguments::get_args();
	if arguments.contains(&Argument::Create) {
		let created_link = write_cli::new_repo();
		process_file(&created_link, &arguments);
		return;
	}
	for messages_file in &links {
		process_file(messages_file, &arguments)
	}
}

pub fn process_file(link: &str, arguments: &[Argument]) {
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
		let public_key = message.public_key_string();
		let username_result = message.get_username();
		let prev_hash = message.prev_hash_string();
		let hash = message.hash_string();
		let signed = message.is_signed();
		let body = &message.body;
		// Print `body` at the end because it could contain spaces, keywords, and who-knows-what (and has an unknown size)
		// Which would make it hard to know when `body` ends meaning anything after it on the same line is harder to parse
		match username_result {
			Some(username) => println!(
				"Username {username} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {body}"
			),
			None => println!(
				"Public_key {public_key} Replying_to_hash {prev_hash} Hash {hash} Properly_signed {signed} Message {body}"
			),
		}
	}
}
