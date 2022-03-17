use ed25519_dalek::*;

use crate::{Error, Message, SerdeParser};

pub fn get_messages(file: &str, parser: &SerdeParser) -> Result<Vec<Message>, Error> {
	fn to_message(x: ([u8; 32], String, [u8; 32], [u8; 32])) -> Option<Message> {
		let public_key = match PublicKey::from_bytes(&x.0) {
			Ok(i) => i,
			Err(_) => return None,
		};
		let message = x.1;
		let signed = match Signature::from_bytes(&[x.2, x.3].concat()) {
			// Combine the two parts of the signature back into one
			Ok(signature) => public_key.verify(message.as_bytes(), &signature).is_ok(),
			Err(_) => false, // If the signature bytes are not a valid signature, it's not properly signed
		};
		Some(Message {
			public_key,
			message,
			signed,
		})
	}

	Ok(get_messages_vec(file, parser)?
		.into_iter()
		.filter_map(to_message)
		.collect())
}

pub fn get_messages_vec(
	file: &str,
	parser: &SerdeParser,
) -> Result<Vec<([u8; 32], String, [u8; 32], [u8; 32])>, Error> {
	use std::io::Read;

	let file = match std::fs::File::open(file) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(i) => i,
	};

	let mut file_slice = Vec::<u8>::new();
	match (&file).read_to_end(&mut file_slice) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(_) => {}
	};

	match parser {
		SerdeParser::Json => match serde_json::from_slice(&file_slice) {
			Err(err) => Err(Error::JsonError(err)),
			Ok(i) => Ok(i),
		},
		SerdeParser::Smile => match serde_smile::from_slice(&file_slice) {
			Err(_) => Err(Error::SmileError), // serde_smile unfortunately does not expose the ErrorKind enum as public so we cannot specify the error
			Ok(i) => Ok(i),
		},
	}
}
