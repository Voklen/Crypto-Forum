use sha2::{Digest, Sha512};

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
	// Message for display and internal logic
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
	pub hash: [u8; 64],
}

#[derive(Debug, PartialEq, Clone)]
pub struct MessageForWriting {
	// Message stored in the file
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

impl MessageForWriting {
	pub fn get_hash(&self) -> [u8; 64] {
		let mut collection_vector = Vec::<u8>::new();
		collection_vector.extend_from_slice(&self.prev_hash);
		collection_vector.extend_from_slice(&self.public_key.to_bytes());
		collection_vector.extend_from_slice(self.message.as_bytes());
		collection_vector.extend_from_slice(&self.signature.to_bytes());

		let hash = Sha512::digest(&collection_vector);
		hash.into()
	}
}

#[derive(Debug)]
pub enum SerdeParser {
	Json,
	Smile,
}
use std::fmt;
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
	pub prev_hash_pt1: [u8; 32],
	pub prev_hash_pt2: [u8; 32],
	pub public_key: [u8; 32],
	pub message: String,
	pub signature_pt1: [u8; 32],
	pub signature_pt2: [u8; 32],
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
