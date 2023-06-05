use crypto_forum::{custom_types::*, *};

use crate::print_messages::*;

#[path = "cli/account_manager.rs"]
mod account_manager;
#[path = "cli/arguments.rs"]
mod arguments;
#[path = "cli/errors.rs"]
mod errors;
#[path = "cli/input.rs"]
mod input;
#[path = "cli/print_messages.rs"]
mod print_messages;
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
	let messages = get_messages(link);

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

fn get_messages(link: &str) -> Vec<Message> {
	let error = match read::get_messages(link) {
		Ok(res) => return res,
		Err(e) => e,
	};
	match error {
		Error::StdIo(e) => throw!("Async tokio error at {link}: {e}"),
		Error::IPFS(e) => throw!("Error reading from IPFS at {link}: {e}"),
		Error::FromUtf8(e) => throw!("Error parsing UTF-8 string from IPFS cat at {link}: {e}"),
		Error::TomlDeserialization(e) => throw!("Error while parsing TOML at {link}: {e}"),
		e => throw!("An unexpected error has occured when reading messages: {e}"),
	}
}

fn interactive_session(messages_file: &str, messages: Vec<Message>) {
	let keypair = account_manager::login("reference/accounts/");
	let last_hash = match messages.last() {
		Some(i) => i.get_hash(),
		None => [0; 64],
	};
	write_cli::interactive_write(messages_file, keypair, last_hash);
}
