use crate::{
	custom_types::{Error, MessageForWriting, SerdeParser},
	read,
	read_serde::{FullFile, MessageInFile},
};

pub fn write_messages(
	file: &str,
	parser: &SerdeParser,
	data: Vec<MessageForWriting>,
) -> Result<(), Error> {
	let write_data = get_write_data(file, parser, data)?;
	// Convert into chosen format
	let value = match parser {
		&SerdeParser::Json => serde_json::to_vec(&write_data).map_err(|err| Error::JsonError(err))?,
		&SerdeParser::Smile => serde_smile::to_vec(&write_data).map_err(|err| Error::SmileError(err))?,
	};
	// Write to file
	std::fs::write(file, value)
		.map_err(|err| Error::StdIo(err))
}

fn get_write_data(
	file: &str,
	parser: &SerdeParser, //TODO make optional argument
	data: Vec<MessageForWriting>,
) -> Result<FullFile, Error> {
	// Read file (see Decisions.md for explanation)
	let file_slice = std::fs::read(file).or_else(handle_error)?;
	let parser = &read::file_type(&file_slice).unwrap();
	// Get messages already in file to concatenate
	let existing_file = crate::read_serde::get_full_file(&file_slice, parser)?;
	// Concatenate old and new messages
	let mut messages = existing_file.messages;
	messages.append(&mut sig_message_to_vec(data));

	Ok(FullFile {
		header: existing_file.header,
		messages,
	})
}

fn handle_error(err: std::io::Error) -> Result<Vec<u8>, Error> {
	if err.kind() == std::io::ErrorKind::NotFound {
		Ok(Vec::<u8>::new())
	} else {
		Err(Error::StdIo(err))
	}
}

macro_rules! split_in_half {
	($e:expr, $size:expr) => ({
		fn to_32(input: [u8; $size*2], offset: usize) -> [u8; $size] {
			let mut out = [0; $size];
			for (i, element) in input.into_iter().enumerate() {
				if offset <= i && i < ($size + offset) {
					out[i - offset] = element;
				}
			}
			out
		}
		(to_32($e, 0), to_32($e, $size))
	});
}

pub fn sig_message_to_vec(data: Vec<MessageForWriting>) -> Vec<MessageInFile> {
	data.into_iter()
		.map(|f| {
			let (prev_hash_pt1, prev_hash_pt2) = split_in_half!(f.prev_hash, 32);
			let (signature_pt1, signature_pt2) = split_in_half!(f.signature.to_bytes(), 32);
			let public_key = f.public_key.to_bytes();
			let message = f.message;
			MessageInFile {
				prev_hash_pt1,
				prev_hash_pt2,
				public_key,
				message,
				signature_pt1,
				signature_pt2,
			}
		})
		.collect()
}
