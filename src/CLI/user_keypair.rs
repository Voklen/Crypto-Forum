use crate::useful_funcs;
use ed25519_dalek::*;

pub fn get_keypair() -> Keypair {
	let secret_seed = get_random_from_usr();

	let secret: SecretKey = SecretKey::from_bytes(&secret_seed[..SECRET_KEY_LENGTH]).unwrap();
	let public: PublicKey = PublicKey::from(&secret);
	Keypair {secret, public}
}

/* Get the user to enter some random characters, then hash whatever they give and return that hash as a byte array */
fn get_random_from_usr() -> [u8; 64] {
	println!(
		"Please type some random characters (this will be used for the initial key generation)"
	);
	let random_input: Result<String, _> = text_io::try_read!("{}\n");

	match random_input {
		Ok(res) => useful_funcs::hash(res.as_bytes()),
		Err(_err) => {
			println!("Sorry, couldn't read the input. Try again.");
			get_random_from_usr()
		}
	}
}
