use crate::{base64::*, custom_types::*};
use ed25519_dalek::*;
use sha2::{Digest, Sha512};

pub fn get_messages(file_slice: &str) -> Result<Vec<Message>, Error> {
	Ok(parse_full_file(file_slice)?
		.messages
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

fn vec_to_message(f: FileMessage) -> Option<Message> {
	let prev_hash_bytes: [u8; 64] = hex_to_bytes(&f.prev_hash)?;
	let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = hex_to_bytes(&f.public_key)?;
	let signature_bytes: [u8; 64] = hex_to_bytes(&f.signature)?;

	let to_hash = {
		let mut result = Vec::<u8>::new();
		result.extend_from_slice(&prev_hash_bytes);
		result.extend_from_slice(&public_key_bytes);
		result.extend_from_slice(f.message.as_bytes());
		result.extend_from_slice(&signature_bytes);
		result
	};

	let prev_hash: [u8; 64] = hex_to_bytes(&f.prev_hash)?;
	let public_key = PublicKey::from_bytes(&public_key_bytes).ok()?;
	let message = f.message;
	let signature = match Signature::from_bytes(&signature_bytes) {
		Ok(i) => Some(i),
		Err(_) => None,
	}?;
	let message = Message {
		prev_hash,
		public_key,
		message,
		signature,
	};

	let hash: [u8; 64] = Sha512::digest(to_hash).into();
	if message.get_hash() == hash {
		Some(message)
	} else {
		None
	}
}

pub fn parse_full_file(file_slice: &str) -> Result<FullFile, Error> {
	if file_slice.is_empty() {
		return Ok(FullFile::new());
	}
	toml::from_str(file_slice).map_err(Error::toml_deserialization)
}
