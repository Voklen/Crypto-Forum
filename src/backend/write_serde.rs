use crate::{
	custom_types::{Error, MessageForWriting, SerdeParser},
	read,
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
	parser: &SerdeParser,
	data: Vec<MessageForWriting>,
) -> Result<Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>, Error> {
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

pub fn sig_message_to_vec(
	data: Vec<MessageForWriting>,
) -> Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])> {
	data.into_iter()
		.map(|f| {
			let (hash_part_1, hash_part_2) = split_in_half!(f.prev_hash, 32);
			let (signature_part_1, signature_part_2) = split_in_half!(f.signature.to_bytes(), 32);
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
