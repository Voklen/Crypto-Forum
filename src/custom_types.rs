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

#[derive(Debug)]
pub struct Message { // Message for display and internal logic
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
	pub hash: [u8; 64],
}

#[derive(Debug)]
pub struct MessageForWriting { // Message stored in the file
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

#[derive(PartialEq)]
pub enum Argument {
	Interactive,
	MachineOutput,
}