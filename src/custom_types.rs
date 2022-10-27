use crate::hex::bytes_to_hex;

use ed25519_dalek::Verifier;
use sha2::{Digest, Sha512};
use std::fmt;

#[derive(Debug)]
pub enum Error {
	StdIo(std::io::Error),
	Encryption(chacha20poly1305::aead::Error),
	SmileError(serde_smile::Error),
	JsonError(serde_json::Error),
	InvalidFileData(String),
	SignatureError(ed25519_dalek::SignatureError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
	// Message stored in the file
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
}

#[derive(Debug)]
pub enum SerdeParser {
	Json,
	Smile,
}

impl fmt::Display for SerdeParser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			SerdeParser::Json => write!(f, "Json"),
			SerdeParser::Smile => write!(f, "Smile"),
		}
	}
}

#[derive(PartialEq)]
pub enum Argument {
	Interactive,
	MachineOutput,
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
pub struct MessageInFile {
	pub prev_hash: String,
	pub public_key: [u8; 32],
	pub message: String,
	pub signature: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct FullFile {
	pub header: Header,
	pub messages: Vec<MessageInFile>,
}

impl FullFile {
	pub fn new() -> Self {
		Self {
			header: Header::new(),
			messages: Vec::<MessageInFile>::new(),
		}
	}
}
