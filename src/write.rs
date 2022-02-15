use ed25519_dalek::*;

pub fn interactive_write(keypair: Keypair) {

	let mut write_data = Vec::<(
		[u8; 32], // Keypair
		String, // Message
		// We have to split up the [u8; 64] into two [u8; 32] as currently serde_smile::from_slice() cannot handle the former
		[u8; 32], // Signature part 1
		[u8; 32], // Signature part 2
	)>::new();

	// THIS BREAKS IF THEIR KEY SEED IS ALL 0'S
	let bad_secret: SecretKey = SecretKey::from_bytes(&[0; SECRET_KEY_LENGTH]).unwrap();
	let bad_public: PublicKey = PublicKey::from(&bad_secret);
	let bad_keypair = Keypair{secret: bad_secret, public: bad_public};

	loop {
		println!("Please enter desired message");
		let message: String = text_io::try_read!().unwrap();

		println!("Would you like to properly sign it? (true/false)");
		if text_io::try_read!().unwrap() {
			let signature: Signature = keypair.sign(message.as_bytes());

			write_data.push((
				keypair.public.to_bytes(), 
				message.to_string(), 
				to_32(signature.to_bytes(), true),
				to_32(signature.to_bytes(), false),
			));
		} else {
			let bad_signature: Signature = bad_keypair.sign(message.as_bytes());

			write_data.push((
				*keypair.public.as_bytes(), 
				message.to_string(), 
				to_32(bad_signature.to_bytes(), true),
				to_32(bad_signature.to_bytes(), false),
			));
		}
		println!("Would you like to enter another message? (true/false)");
		let res: bool = text_io::try_read!().unwrap();
		if !res {
			write_to_smile("test_data/succeed.sml", write_data);
			break;
		}
	}
}

pub fn write_to_smile(file: &str, data: Vec<([u8; 32], String, [u8; 32], [u8; 32])>) {
	use std::io::Write;

	let orig_messages = crate::read::get_messages_vec(file)
		.unwrap_or(Vec::new());
	let final_write_data = [orig_messages,data].concat();
	let value = serde_smile::to_vec(&final_write_data).unwrap();

	let mut file = std::fs::File::create(file).unwrap();
	file.write_all(&value);
}

/* Hope to eventually replace this with a better way, probably including slices */
fn to_32(input: [u8; 64], first_32: bool) -> [u8; 32] {
	let offset = if first_32 {0} else {32};
	let mut out = [0; 32];
	for (i, element) in input.iter().enumerate() {
		if offset <= i && i < (32 + offset) {
			out[i - offset] = *element;
		}
	}
	out
}