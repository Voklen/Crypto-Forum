use crate::custom_types::*;
use std::fs;

pub fn write_messages(
	file: &str,
	parser: &SerdeParser,
	data: Vec<MessageForWriting>,
) -> Result<Vec<u8>, Error> {
	let write_data = get_write_data(file, parser, data)?;
	// Convert into chosen format
	let value = match *parser {
		SerdeParser::Json => serde_json::to_vec(&write_data).map_err(Error::JsonError)?,
		SerdeParser::Smile => serde_smile::to_vec(&write_data).map_err(Error::SmileError)?,
	};

	fs::write(file, &value).map_err(Error::StdIo)?;
	Ok(value)
}

fn get_write_data(
	file: &str,
	parser: &SerdeParser,
	data: Vec<MessageForWriting>,
) -> Result<FullFile, Error> {
	let mut new_messages = sig_message_to_vec(data);

	// Read existing messages (see Decisions.md for explanation)
	let existing_file = get_full_file(file, parser)?;
	let mut messages = existing_file.messages;
	messages.append(&mut new_messages);

	Ok(FullFile {
		header: existing_file.header,
		messages,
	})
}

fn get_full_file(file: &str, parser: &SerdeParser) -> Result<FullFile, Error> {
	match fs::read(file) {
		Ok(file_slice) => crate::read_serde::parse_full_file(&file_slice, parser),
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

pub fn sig_message_to_vec(data: Vec<MessageForWriting>) -> Vec<MessageInFile> {
	data.into_iter()
		.map(|f| {
			let prev_hash = hex::bytes_to_hex(&f.prev_hash);
			let signature = hex::bytes_to_hex(&f.signature.to_bytes());
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
