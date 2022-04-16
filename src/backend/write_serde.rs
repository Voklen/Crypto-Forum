use crate::{read, Error, SerdeParser, MessageForWriting};

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
		.or_else(|err| Err(Error::StdIo(err)))
}

fn get_write_data(file: &str, parser: &SerdeParser, data: Vec<MessageForWriting>) -> Result<Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>, Error> {
    // Read file (see Decisions.md for explanation)
	let file_slice = match read::read_file_data(file) {
		Ok((slice, _)) => slice,
		Err(Error::StdIo(err)) if err.kind() == std::io::ErrorKind::NotFound => Vec::<u8>::new(),
		Err(err) => return Err(err),
	};
	// Get messages already in file to concatenate
	let orig_messages = crate::read_serde::get_messages_vec(&file_slice, parser)?;
	// Concatenate old and new messages
	Ok([orig_messages, sig_message_to_vec(data)].concat())
}

pub fn sig_message_to_vec(data: Vec<MessageForWriting>) -> Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])> {
	data.into_iter()
		.map(|f| {
			let (hash_part_1, hash_part_2) = f.prev_hash.split_at(32);
			let (signature_part_1, signature_part_2) = f.signature.split_at(32);
			(
				hash_part_1,
				hash_part_2,
				f.public_key.to_bytes(),
				f.message,
				signature_part_1,
				signature_part_2,
			)
		})
		.collect()
}
