use crate::{read, Error, SerdeParser, SignatureMessage};

pub fn write_to_serde(
	file: &str,
	parser: &SerdeParser,
	data: Vec<SignatureMessage>,
) -> Result<(), Error> {
	use std::io::Write;

	// Read file (see Decisions.md for explanation)
	let file_slice = match read::read_file_data(file) {
		Ok(i) => i.0,
		Err(Error::StdIo(std::io::ErrorKind::NotFound)) => Vec::<u8>::new(),
		Err(err) => return Err(err),
	};
	// Get messages already in file to concatenate
	let orig_messages = crate::read_serde::get_messages_vec(&file_slice, parser)
		.unwrap_or(Vec::<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>::new());
	// Concatenate old and new messages
	let write_data = [orig_messages, sig_message_to_vec(data)].concat();

	// Convert into chosen format
	let value = match parser {
		&SerdeParser::Json => match serde_json::to_vec(&write_data) {
			Ok(i) => i,
			Err(err) => return Err(Error::JsonError(err)),
		},
		&SerdeParser::Smile => match serde_smile::to_vec(&write_data) {
			Ok(i) => i,
			Err(_) => return Err(Error::SmileError),
		},
	};

	// Write to file
	let mut file = match std::fs::File::create(file) {
		Ok(i) => i,
		Err(err) => return Err(Error::StdIo(err.kind())),
	};
	file.write_all(&value)
		.or_else(|err| {
			Err(Error::StdIo(err.kind()))
		})
}

pub fn sig_message_to_vec(data: Vec<SignatureMessage>) -> Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])> {
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
	for (i, element) in input.iter().enumerate() {
		if offset <= i && i < (32 + offset) {
			out[i - offset] = *element;
		}
	}
	out
}
