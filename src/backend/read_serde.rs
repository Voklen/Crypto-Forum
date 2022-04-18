use crate::{Error, Message, SerdeParser};
use ed25519_dalek::*;
use sha2::{Digest, Sha512};

pub fn get_messages(file_slice: &Vec<u8>, parser: &SerdeParser) -> Result<Vec<Message>, Error> {
	Ok(get_messages_vec(file_slice, parser)?
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

fn vec_to_message(f: ([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])) -> Option<Message> {
	let to_hash = [&f.0, &f.1, &f.2, f.3.as_bytes(), &f.4, &f.5].concat();
	let hash = Sha512::digest(to_hash).into();

	let prev_hash: [u8; 64] = our_append(f.0, f.1);
	let public_key = PublicKey::from_bytes(&f.2).ok()?;
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

pub fn get_messages_vec(
	file_slice: &Vec<u8>,
	parser: &SerdeParser,
) -> Result<Vec<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>, Error> {
	if file_slice.len() <= 0 {
		return Ok(Vec::<([u8; 32], [u8; 32], [u8; 32], String, [u8; 32], [u8; 32])>::new());
	}
	match parser {
		SerdeParser::Json => serde_json::from_slice(&file_slice).map_err(|err|Error::JsonError(err)),
		SerdeParser::Smile => serde_smile::from_slice(&file_slice).map_err(|err| Error::SmileError(err)),
	}
}

fn our_append(first: [u8; 32], second: [u8; 32]) -> [u8; 64] {
	let mut output = [0; 64];
	output[..32].copy_from_slice(first.as_slice());
	output[32..].copy_from_slice(second.as_slice());
	output
}
