use crate::base64::bytes_to_hex;

use ed25519_dalek::Verifier;
use sha2::{Digest, Sha512};
use std::fs;

#[derive(Debug)]
pub enum Error {
	StdIo(std::io::Error),
	Encryption(chacha20poly1305::aead::Error),
	TomlSerialization(toml::ser::Error),
	TomlDeserialization(toml::de::Error),
	InvalidFileData(String),
	SignatureError(ed25519_dalek::SignatureError),
	IPFS(ipfs_api_backend_hyper::Error),
}

impl Error {
	pub fn toml_serialization(error: toml::ser::Error) -> Error {
		Error::TomlSerialization(error)
	}

	pub fn toml_deserialization(error: toml::de::Error) -> Error {
		Error::TomlDeserialization(error)
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

impl Message {
	// Logic methods
	pub fn get_hash(&self) -> [u8; 64] {
		let mut collection_vector = Vec::<u8>::new();
		collection_vector.extend_from_slice(&self.prev_hash);
		collection_vector.extend_from_slice(&self.public_key.to_bytes());
		collection_vector.extend_from_slice(self.message.as_bytes());
		collection_vector.extend_from_slice(&self.signature.to_bytes());

		let hash = Sha512::digest(&collection_vector);
		hash.into()
	}

	pub fn is_signed(&self) -> bool {
		let combined_data = &[self.message.as_bytes(), &self.prev_hash].concat();
		self.public_key
			.verify(combined_data, &self.signature)
			.is_ok()
	}

	// Display methods
	pub fn hex_prev_hash(&self) -> String {
		let bytes = &self.prev_hash;
		bytes_to_hex(bytes)
	}

	pub fn hex_public_key(&self) -> String {
		let bytes = self.public_key.as_bytes();
		bytes_to_hex(bytes)
	}

	pub fn hex_signature(&self) -> String {
		let bytes = &self.signature.to_bytes();
		bytes_to_hex(bytes)
	}

	pub fn hex_hash(&self) -> String {
		let bytes = &self.get_hash();
		bytes_to_hex(bytes)
	}

	pub fn get_username(&self) -> Option<String> {
		let public_key = self.hex_public_key();
		let usernames_dir = "reference/usernames/";
		let username_file = usernames_dir.to_owned() + &public_key;
		let result = match fs::read_to_string(username_file) {
			Ok(res) => res,
			Err(_) => return None,
		};
		Some(result.trim().to_string())
	}
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Header {
	pub name: String,
	pub thread_number: u32,
	pub tags: Vec<String>,
}

impl Header {
	pub fn new() -> Self {
		Self {
			name: String::new(),
			thread_number: 0,
			tags: Vec::<String>::new(),
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileMessage {
	pub prev_hash: String,
	pub public_key: String,
	pub message: String,
	pub signature: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct FullFile {
	pub header: Header,
	pub messages: Vec<FileMessage>,
}

impl FullFile {
	pub fn new() -> Self {
		Self {
			header: Header::new(),
			messages: Vec::<FileMessage>::new(),
		}
	}
}

pub fn ipns_link_to_key(link: &str) -> String {
	format!("GitArkRepo{link}")
}
