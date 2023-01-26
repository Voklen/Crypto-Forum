use crate::custom_types::Error;
use chacha20poly1305::{
	aead::{Aead, KeyInit},
	XChaCha20Poly1305,
};
use std::fs;

pub fn encrypt_and_write(file: &str, data_to_encrypt: &[u8], key: &[u8; 32]) -> Result<(), Error> {
	use rand::{rngs::OsRng, RngCore};

	// Randomise nonce
	let mut nonce = [0; 24];
	OsRng.fill_bytes(&mut nonce);

	let mut encrypted_data = XChaCha20Poly1305::new(key.into())
		.encrypt(&nonce.into(), data_to_encrypt)
		.map_err(Error::Encryption)?;

	// Add nonce to the beginning of the encrypted data for when we write to the file
	let mut output = nonce.to_vec();
	output.append(&mut encrypted_data);

	fs::write(file, output).map_err(Error::StdIo)
}

pub fn read_and_decrypt(file: &str, key: &[u8; 32]) -> Result<Vec<u8>, Error> {
	let file_data = fs::read(file).map_err(Error::StdIo)?;

	// Extract the first 24 bytes as the nonce
	if file_data.len() <= 24 {
		return Err(Error::InvalidFileData(file.to_string()));
	}
	let (nonce, encrypted_data) = file_data.split_at(24);

	// Decrypt
	XChaCha20Poly1305::new(key.into())
		.decrypt(nonce.into(), encrypted_data)
		.map_err(Error::Encryption)
}
