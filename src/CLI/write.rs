use ed25519_dalek::*;
use crate::write_smile;

pub fn interactive_write(file: &str, keypair: Keypair) {
	let write_data = Vec::<(
		[u8; 32], // Keypair
		String,   // Message
		// We have to split up the [u8; 64] into two [u8; 32] as currently serde_smile::from_slice() cannot handle the former
		[u8; 32], // Signature part 1
		[u8; 32], // Signature part 2
	)>::new();

	// THIS BREAKS IF THEIR KEY SEED IS ALL 0'S
	let bad_secret: SecretKey = SecretKey::from_bytes(&[0; SECRET_KEY_LENGTH]).unwrap();
	let bad_public: PublicKey = PublicKey::from(&bad_secret);
	let bad_keypair = Keypair {
		secret: bad_secret,
		public: bad_public,
	};

	let messages = get_messages_from_user(keypair, write_data, bad_keypair);
	write_smile::write_to_smile(file, messages)
}

fn get_messages_from_user(
	keypair: Keypair,
	mut write_data: Vec<([u8; 32], String, [u8; 32], [u8; 32])>,
	bad_keypair: Keypair,
) -> Vec<([u8; 32], std::string::String, [u8; 32], [u8; 32])> {
	println!("Please enter desired message");
	let message: String = text_io::try_read!().unwrap();
	println!("Would you like to properly sign it? (true/false)");
	let signature: Signature = 
		if text_io::try_read!().unwrap() {
			keypair.sign(message.as_bytes())
		} else {
			bad_keypair.sign(message.as_bytes())
		};
	
	let new_element = (
		keypair.public.to_bytes(),
		message.to_string(),
		write_smile::to_32(signature.to_bytes(), true),
		write_smile::to_32(signature.to_bytes(), false),
	);
	write_data.push(new_element);

	println!("Would you like to enter another message? (true/false)");
	let res: bool = text_io::try_read!().unwrap();
	if !res {
		return write_data;
	}
	get_messages_from_user(keypair, write_data, bad_keypair)
}
