use crate::custom_types::*;
use ed25519_dalek::*;
use sha2::{Digest, Sha512};

pub fn get_messages(file_slice: &Vec<u8>, parser: &SerdeParser) -> Result<Vec<Message>, Error> {
	Ok(parse_full_file(file_slice, parser)?
		.messages
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

fn vec_to_message(f: MessageInFile) -> Option<Message> {
	let to_hash = {
		let mut result = Vec::<u8>::new();
		let prev_hash_bytes: [u8; 64] = hex::hex_to_bytes(&f.prev_hash);
		result.extend_from_slice(&prev_hash_bytes);
		result.extend_from_slice(&f.public_key);
		result.extend_from_slice(f.message.as_bytes());
		let signature_bytes: [u8; 64] = hex::hex_to_bytes(&f.signature);
		result.extend_from_slice(&signature_bytes);
		result
	};
	let hash = Sha512::digest(to_hash).into();

	let prev_hash: [u8; 64] = hex::hex_to_bytes(&f.prev_hash);
	let public_key = PublicKey::from_bytes(&f.public_key).ok()?;
	let message = f.message;
	let signature_bytes: [u8; 64] = hex::hex_to_bytes(&f.signature);
	let signed = match Signature::from_bytes(&signature_bytes) {
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

pub fn parse_full_file(file_slice: &Vec<u8>, parser: &SerdeParser) -> Result<FullFile, Error> {
	if file_slice.is_empty() {
		return Ok(FullFile::new());
	}
	match parser {
		SerdeParser::Json => serde_json::from_slice(file_slice).map_err(Error::JsonError),
		SerdeParser::Smile => serde_smile::from_slice(file_slice).map_err(Error::SmileError),
	}
}
