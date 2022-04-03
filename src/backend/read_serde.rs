use ed25519_dalek::*;

use crate::{Error, Message, SerdeParser};

pub fn get_messages(file_slice: &Vec<u8>, parser: &SerdeParser) -> Result<Vec<Message>, Error> {
	fn to_message(x: ([u8; 32], String, [u8; 32], [u8; 32], [u8; 32], [u8; 32])) -> Option<Message> {
		let public_key = match PublicKey::from_bytes(&x.0) {
			Ok(i) => i,
			Err(_) => return None,
		};
		let prev_hash = [0; 64];
		let message = x.1;
		let signed = match Signature::from_bytes(&[x.2, x.3].concat()) {
			// Combine the two parts of the signature back into one
			Ok(signature) => {
				let to_verify = &[message.as_bytes(), &prev_hash].concat();
				public_key.verify(to_verify, &signature).is_ok()
			}
			Err(_) => false, // If the signature bytes are not a valid signature, it's not properly signed
		};
		Some(Message {
			public_key,
			message,
			signed,
		})
	}

	Ok(get_messages_vec(file_slice, parser)?
		.into_iter()
		.filter_map(to_message)
		.collect())
}

pub fn get_messages_vec(
	file_slice: &Vec<u8>,
	parser: &SerdeParser,
) -> Result<Vec<([u8; 32], String, [u8; 32], [u8; 32], [u8; 32], [u8; 32])>, Error> {
	match parser {
		SerdeParser::Json => match serde_json::from_slice(&file_slice) {
			Err(err) => {
				if file_slice == "[[]]".as_bytes() {
					// If the error is due to the file being an empty json, return an empty vector
					Ok(Vec::<([u8; 32], String, [u8; 32], [u8; 32], [u8; 32], [u8; 32])>::new())
				} else {
					Err(Error::JsonError(err))
				}
			}
			Ok(i) => Ok(i),
		},
		SerdeParser::Smile => match serde_smile::from_slice(&file_slice) {
			Err(_) => Err(Error::SmileError), // serde_smile unfortunately does not expose the ErrorKind enum as public so we cannot specify the error
			Ok(i) => Ok(i),
		},
	}
}
