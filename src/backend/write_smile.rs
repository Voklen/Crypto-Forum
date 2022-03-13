use crate::{Error, SignatureMessage};

pub fn write_to_smile(file: &str, data: Vec<SignatureMessage>) -> Result<(), Error> {
	use std::io::Write;

	let orig_messages = match crate::read_smile::get_messages_vec(file) {
		Ok(i) => i,
		Err(_) => Vec::<([u8; 32], String, [u8; 32], [u8; 32])>::new(),
	};
	let write_data = [orig_messages, sig_message_to_vec(data)].concat();
	let value = match serde_smile::to_vec(&write_data) {
		Ok(i) => i,
		Err(_) => return Err(Error::SmileError),
	};

	let mut file = std::fs::File::create(file).unwrap();
	match file.write_all(&value) {
		Ok(_) => Ok(()),
		Err(err) => Err(Error::StdIo(err.kind())),
	}
}

fn sig_message_to_vec(data: Vec<SignatureMessage>) -> Vec<([u8; 32], String, [u8; 32], [u8; 32])> {
	data.into_iter()
		.map(|f| {
			(
				f.public_key.to_bytes(),
				f.message,
				to_32(f.signature.to_bytes(), true),
				to_32(f.signature.to_bytes(), false),
			)
		})
		.collect()
}

/* Hope to eventually replace this with a better way, probably including slices */
pub fn to_32(input: [u8; 64], first_32: bool) -> [u8; 32] {
	let offset = if first_32 { 0 } else { 32 };
	let mut out = [0; 32];
	for (i, element) in input.iter().enumerate() {
		if offset <= i && i < (32 + offset) {
			out[i - offset] = *element;
		}
	}
	out
}
