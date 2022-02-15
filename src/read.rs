use std::io::Read;

use ed25519_dalek::*;
#[derive(Debug)]
pub struct Message {
    pub public_key: PublicKey,
    pub message: String,
    pub signed: bool,
}

pub fn get_messages(file: &str) -> Vec<Message> {
	
	get_messages_vec(file)
	.unwrap()
	.into_iter()
	.map(|x|{
		let public_key = PublicKey::from_bytes(&x.0).unwrap();
		let message = x.1;
		let signature = Signature::from_bytes(&[x.2, x.3].concat()).unwrap(); // Combine the two parts of the signature back into one
		let signed = public_key.verify(message.as_bytes(), &signature).is_ok();
		Message{
			public_key,
			message,
			signed,
		}
	})
	.collect()
}

pub fn get_messages_vec(file: &str) -> Result<Vec<([u8; 32], String, [u8; 32], [u8; 32])>, std::io::Error> {
	let mut file = std::fs::File::open(file)?;
	let mut smile = Vec::<u8>::new();
	file.read_to_end(&mut smile);

	let res: Vec<([u8; 32], String, [u8; 32], [u8; 32])> = serde_smile::from_slice(&smile).unwrap();
	Ok(res)
}
