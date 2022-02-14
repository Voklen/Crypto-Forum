use std::io::Read;

use ed25519_dalek::*;
#[derive(Debug)]
struct Message {
    public_key: PublicKey,
    message: String,
    signature: Signature,
}

pub fn main() -> Vec<&'static str>{
	println!("Hello");

	let messages = get_messages("test_data/succeed.sml");
	let verified = verify_messages(messages);

	vec!["hello"]
}

fn get_messages(file: &str) -> Vec<Message> {
	
	get_messages_vec(file)
	.into_iter()
	.map(|x|
	Message{
		public_key: PublicKey::from_bytes(&x.0).unwrap(),
		message: x.1,
		signature: Signature::from_bytes(&[x.2, x.3].concat()).unwrap() // Combine the two parts of the signature back into one
	})
	.collect()
}

pub fn get_messages_vec(file: &str) -> Vec<([u8; 32], String, [u8; 32], [u8; 32])> {
	let mut file = std::fs::File::open(file).unwrap();
	let mut smile = Vec::<u8>::new();
	file.read_to_end(&mut smile);

	let res: Vec<([u8; 32], String, [u8; 32], [u8; 32])> = serde_smile::from_slice(&smile).unwrap();
	res
}

fn verify_messages(messages: Vec<Message>) -> Vec<bool> {
	messages
	.into_iter()
	.map(|x|{
		x.public_key.verify(x.message.as_bytes(), &x.signature).is_ok()
	})
	.collect()
}
