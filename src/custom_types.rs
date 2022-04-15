use crate::useful_funcs;

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
pub struct SignatureMessage { // Message stored in the file
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

impl SignatureMessage {
	pub fn get_hash(&self) -> [u8; 64] {
		let mut collection_vector = Vec::<u8>::new();
		collection_vector.extend_from_slice(&self.prev_hash);
		collection_vector.extend_from_slice(&self.public_key.to_bytes());
		collection_vector.extend_from_slice(self.message.as_bytes());
		collection_vector.extend_from_slice(&self.signature.to_bytes());

		useful_funcs::hash(&collection_vector)
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