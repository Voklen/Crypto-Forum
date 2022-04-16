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
	let orig_messages = crate::read_serde::get_messages_vec(&file_slice, parser).unwrap();
	// Concatenate old and new messages
	Ok([orig_messages, sig_message_to_vec(data)].concat())
}

pub fn sig_message_to_vec(data: Vec<MessageForWriting>) -> Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])> {
	data.into_iter()
		.map(|f| {
			(
				to_32(f.prev_hash, true),
				to_32(f.prev_hash, false),
				f.public_key.to_bytes(),
				f.message,
				to_32(f.signature.to_bytes(), true),
				to_32(f.signature.to_bytes(), false),
			)
		})
		.collect()
}

/* Hope to eventually replace this with a better way, probably including slices */
fn to_32(input: [u8; 64], first_32: bool) -> [u8; 32] {
	let offset = if first_32 { 0 } else { 32 };
	let mut out = [0; 32];
	for (i, element) in input.into_iter().enumerate() {
		if offset <= i && i < (32 + offset) {
			out[i - offset] = element;
		}
	}
	out
}
