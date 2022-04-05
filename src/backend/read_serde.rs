use ed25519_dalek::*;

use crate::{Error, Message, SerdeParser, useful_funcs};

pub fn get_messages(file_slice: &Vec<u8>, parser: &SerdeParser) -> Result<Vec<Message>, Error> {
	fn to_message(f: ([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])) -> Option<Message> {
		// Calculate hash by borrowing values before they're moved (rust memory management)
		let hash = useful_funcs::hash(&[&f.0, &f.1, &f.2, f.3.as_bytes(), &f.4, &f.5].concat());

		let prev_hash: [u8; 64] = our_append(f.0, f.1);
		let public_key = match PublicKey::from_bytes(&f.2) {
			Ok(i) => i,
			Err(_) => return None,
		};
		let message = f.3;
		let signed = match Signature::from_bytes(&[f.4, f.5].concat()) {
			// Combine the two parts of the signature back into one
			Ok(signature) => {
				let to_verify = &[message.as_bytes(), &prev_hash].concat();
				public_key.verify(to_verify, &signature).is_ok()
			}
			Err(_) => false, // If the signature bytes are not a valid signature, it's not properly signed
		};
		Some(Message {
			prev_hash,
			public_key,
			message,
			signed,
			hash,
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
) -> Result<Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>, Error> {
	match parser {
		SerdeParser::Json => match serde_json::from_slice(&file_slice) {
			Err(err) => {
				if file_slice == "[[]]".as_bytes() {
					// If the error is due to the file being an empty json, return an empty vector
					Ok(Vec::<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>::new())
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

fn our_append(first: [u8; 32], second: [u8; 32]) -> [u8; 64] {
	let mut output = [0; 64];
	output[..32].copy_from_slice(first.as_slice());
	output[32..].copy_from_slice(second.as_slice());
	output
}
