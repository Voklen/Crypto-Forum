use crate::custom_types::*;
use base64::{engine::general_purpose, Engine};
use ed25519_dalek::*;
use futures::TryStreamExt;
use ipfs_api_backend_hyper::{IpfsApi, IpfsClient};
use sha2::{Digest, Sha512};

pub fn get_messages(link: &str) -> Result<Vec<Message>, Error> {
	Ok(get_repo(link)?
		.messages
		.into_iter()
		.filter_map(vec_to_message)
		.collect())
}

pub fn get_repo(link: &str) -> Result<FullFile, Error> {
	let file_slice = read_file(link)?;
	if file_slice.is_empty() {
		return Ok(FullFile::new());
	}
	toml::from_str(&file_slice).map_err(Error::TomlDeserialization)
}

pub fn read_file(ipns_link: &str) -> Result<String, Error> {
	let client = IpfsClient::default();
	let executor = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.map_err(Error::StdIo)?;

	let ipfs_link_future = client.name_resolve(Some(ipns_link), true, false);
	let ipfs_link = executor.block_on(ipfs_link_future).map_err(Error::IPFS)?;
	let content_future = client
		.cat(&ipfs_link.path)
		.map_ok(|chunk| chunk.to_vec())
		.try_concat();
	let content = executor.block_on(content_future).map_err(Error::IPFS)?;
	String::from_utf8(content).map_err(Error::FromUtf8)
}

fn vec_to_message(f: FileMessage) -> Option<Message> {
	let prev_hash_bytes: [u8; 64] = decode_base64(&f.prev_hash)?;
	let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = decode_base64(&f.public_key)?;
	let signature_bytes: [u8; 64] = decode_base64(&f.signature)?;

	let to_hash = {
		let mut result = Vec::<u8>::new();
		result.extend_from_slice(&prev_hash_bytes);
		result.extend_from_slice(&public_key_bytes);
		result.extend_from_slice(f.body.as_bytes());
		result.extend_from_slice(&signature_bytes);
		result
	};

	let prev_hash: [u8; 64] = decode_base64(&f.prev_hash)?;
	let public_key = PublicKey::from_bytes(&public_key_bytes).ok()?;
	let body = f.body;
	let signature = match Signature::from_bytes(&signature_bytes) {
		Ok(i) => Some(i),
		Err(_) => None,
	}?;
	let message = Message {
		prev_hash,
		public_key,
		body,
		signature,
	};

	let hash: [u8; 64] = Sha512::digest(to_hash).into();
	if message.get_hash() == hash {
		Some(message)
	} else {
		None
	}
}

fn decode_base64<const N: usize>(string: &str) -> Option<[u8; N]> {
	// I want to use `decode_slice` for performance but it was not working
	// maybe have another go sometime
	let base64 = general_purpose::STANDARD_NO_PAD;
	let as_vec = base64.decode(string).ok()?;
	as_vec.try_into().ok()
}
