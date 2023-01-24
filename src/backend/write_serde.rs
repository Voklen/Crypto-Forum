use crate::{custom_types::*, hex::*, read_serde};
use std::fs;

pub fn write_messages(file: &str, data: Vec<Message>) -> Result<Vec<u8>, Error> {
	let write_data = get_write_data(file, data)?;
	// Convert into chosen format
	let value = serde_json::to_vec(&write_data).map_err(from_json)?;

	fs::write(file, &value).map_err(Error::StdIo)?;
	Ok(value)
}

fn get_write_data(file: &str, data: Vec<Message>) -> Result<FullFile, Error> {
	let mut new_messages = sig_message_to_vec(data);

	// Read existing messages (see Decisions.md for explanation)
	let existing_file = get_full_file(file)?;
	let mut messages = existing_file.messages;
	messages.append(&mut new_messages);

	Ok(FullFile {
		header: existing_file.header,
		messages,
	})
}

fn get_full_file(file: &str) -> Result<FullFile, Error> {
	match fs::read(file) {
		Ok(file_slice) => read_serde::parse_full_file(&file_slice).map_err(from_json),
		Err(err) => handle_error(err),
	}
}

fn handle_error(err: std::io::Error) -> Result<FullFile, Error> {
	if err.kind() == std::io::ErrorKind::NotFound {
		Ok(FullFile::new())
	} else {
		Err(Error::StdIo(err))
	}
}

pub fn sig_message_to_vec(data: Vec<Message>) -> Vec<MessageInFile> {
	data.into_iter()
		.map(|f| {
			let prev_hash = bytes_to_hex(&f.prev_hash);
			let signature = bytes_to_hex(&f.signature.to_bytes());
			let public_key = f.public_key.to_bytes();
			let message = f.message;
			MessageInFile {
				prev_hash,
				public_key,
				message,
				signature,
			}
		})
		.collect()
}
