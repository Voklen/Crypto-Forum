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
	let to_hash = [
		&f.prev_hash_pt1,
		&f.prev_hash_pt2,
		&f.public_key,
		f.message.as_bytes(),
		&f.signature_pt1,
		&f.signature_pt2,
	]
	.concat();
	let hash = Sha512::digest(to_hash).into();

	let prev_hash: [u8; 64] = our_append(f.prev_hash_pt1, f.prev_hash_pt2);
	let public_key = PublicKey::from_bytes(&f.public_key).ok()?;
	let message = f.message;
	let signed = match Signature::from_bytes(&[f.signature_pt1, f.signature_pt2].concat()) {
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

fn our_append(first: [u8; 32], second: [u8; 32]) -> [u8; 64] {
	let mut output = [0; 64];
	output[..32].copy_from_slice(first.as_slice());
	output[32..].copy_from_slice(second.as_slice());
	output
}
