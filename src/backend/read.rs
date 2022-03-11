use ed25519_dalek::*;

#[derive(Debug, PartialEq)]
pub enum Error {
	StdIo(std::io::ErrorKind),
	SmileError,
}

#[derive(Debug)]
pub struct Message {
	pub public_key: PublicKey,
	pub message: String,
	pub signed: bool,
}

pub fn get_messages(file: &str) -> Result<Vec<Message>, Error> {
	fn to_message(x: ([u8; 32], String, [u8; 32], [u8; 32])) -> Option<Message> {
		let public_key = match PublicKey::from_bytes(&x.0) {
			Ok(i) => i,
			Err(_) => return None,
		};
		let message = x.1;
		let signed = match Signature::from_bytes(&[x.2, x.3].concat()) {
			// Combine the two parts of the signature back into one
			Ok(signature) => public_key.verify(message.as_bytes(), &signature).is_ok(),
			Err(_) => false, // If the signature bytes are not a valid signature, it's not properly signed
		};
		Some(Message {
			public_key,
			message,
			signed,
		})
	}

	Ok(get_messages_vec(file)?
		.into_iter()
		.filter_map(to_message)
		.collect())
}

pub fn get_messages_vec(file: &str) -> Result<Vec<([u8; 32], String, [u8; 32], [u8; 32])>, Error> {
	use std::io::Read;

	let file = match std::fs::File::open(file) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(i) => i,
	};
	let mut smile = Vec::<u8>::new();

	match (&file).read_to_end(&mut smile) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(i) => i,
	};

	Ok(match serde_smile::from_slice(&smile) {
		Err(_) => return Err(Error::SmileError), // serde_smile unfortunately does not expose the ErrorKind enum as public so we cannot specify the error
		Ok(i) => i,
	})
}
