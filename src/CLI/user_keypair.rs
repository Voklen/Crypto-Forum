use crate::{useful_funcs, Error};
use ed25519_dalek::*;

pub fn login() -> Result<Keypair, Error> {
	let accounts_file_str = "accounts.json";
	let accounts_file = std::path::PathBuf::from(accounts_file_str);
	if accounts_file.exists() {
		let file_data = read_and_decrypt(accounts_file_str, &get_password())?;
		Keypair::from_bytes(&file_data)
			.map_err(|err| Error::SignatureError(err))
	} else {
		Ok(get_keypair())
	}
}

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

fn get_password() -> [u8; 32] {
	[0; 32]
}

fn encrypt_and_write(file: &str, data_to_encrypt: &[u8], key: &[u8; 32]) -> Result<(), Error> {
	use chacha20poly1305::{aead::{Aead, NewAead}, XChaCha20Poly1305};
	use rand::{rngs::OsRng, RngCore};

	let cipher = XChaCha20Poly1305::new(key.into());

	let mut nonce = [0; 24];
	OsRng.fill_bytes(&mut nonce);

	let mut encrypted_data = cipher
		.encrypt(&nonce.into(), data_to_encrypt)
		.map_err(|err| Error::Encryption(err))?;

	let mut output = nonce.to_vec();
	output.append(&mut encrypted_data);

	std::fs::write(file, output).map_err(|err| Error::StdIo(err.kind()))
}

fn read_and_decrypt(file: &str, key: &[u8; 32]) -> Result<Vec<u8>, Error> {
	use chacha20poly1305::{aead::{Aead, NewAead}, XChaCha20Poly1305};

	let cipher = XChaCha20Poly1305::new(key.into());

	let file_data = std::fs::read(file).map_err(|err| Error::StdIo(err.kind()))?;

	// Will replace this with a better method later (custom split_at! macro that returns an Option)
	if file_data.len() <= 24 {
		return Err(Error::InvalidFileData(file.to_string()));
	}
	let (nonce, encrypted_data) = file_data.split_at(24);

	cipher
		.decrypt(nonce.into(), encrypted_data)
		.map_err(|err| Error::Encryption(err))
}
