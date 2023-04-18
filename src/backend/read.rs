use crate::{base64::*, custom_types::*};
use ed25519_dalek::*;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use sha2::{Digest, Sha512};

pub fn get_messages(link: &str) -> Result<Vec<Message>, Error> {
	Ok(parse_full_file(link)?
		.messages
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

pub fn parse_full_file(link: &str) -> Result<FullFile, Error> {
	let file_slice = read_file(link);
	if file_slice.is_empty() {
		return Ok(FullFile::new());
	}
	toml::from_str(&file_slice).map_err(Error::toml_deserialization)
}

pub fn read_file(link: &str) -> String {
	let client = IpfsClient::default();
	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();
	let result = runtime.block_on(client.get(link).map_ok(|chunk| chunk.to_vec()).try_concat());
	match result {
		Ok(res) => clean_ipfs_cat(res),
		Err(e) => panic!("IPFS retreval error: {}", e),
	}
}

fn clean_ipfs_cat(mut cat_vec: Vec<u8>) -> String {
	cat_vec.drain(..512);
	String::from_utf8(cat_vec)
		.unwrap()
		.trim_end_matches(char::from(0))
		.to_owned()
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
