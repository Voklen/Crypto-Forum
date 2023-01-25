use crate::{custom_types::*, hex::*};
use ed25519_dalek::*;
use sha2::{Digest, Sha512};

pub fn get_messages(file_slice: &str) -> serde_json::Result<Vec<Message>> {
	Ok(parse_full_file(file_slice)
		.messages
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

fn vec_to_message(f: MessageInFile) -> Option<Message> {
	let to_hash = {
		let mut result = Vec::<u8>::new();
		let prev_hash_bytes: [u8; 64] = hex_to_bytes(&f.prev_hash)?;
		result.extend_from_slice(&prev_hash_bytes);
		result.extend_from_slice(&f.public_key);
		result.extend_from_slice(f.message.as_bytes());
		let signature_bytes: [u8; 64] = hex_to_bytes(&f.signature)?;
		result.extend_from_slice(&signature_bytes);
		result
	};

	let prev_hash: [u8; 64] = hex_to_bytes(&f.prev_hash)?;
	let public_key = PublicKey::from_bytes(&f.public_key).ok()?;
	let message = f.message;
	let signature_bytes: [u8; 64] = hex_to_bytes(&f.signature)?;
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

pub fn parse_full_file(file_slice: &str) -> FullFile {
	if file_slice.is_empty() {
		return FullFile::new();
	}
	toml::from_str(file_slice).unwrap()
}
