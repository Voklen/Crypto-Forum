use crate::{
	encrypt_decrypt::{encrypt_and_write, read_and_decrypt},
	useful_funcs, Error,
};
use ed25519_dalek::*;
use sha2::{Digest, Sha256};

pub fn login(accounts_dir: &str) -> Result<Keypair, Error> {
	println!("Do you want to create a new account? (true/false)");
	match text_io::try_read!() {
		Ok(true) => create_account(accounts_dir),
		Ok(false) => get_existing_account(accounts_dir),
		Err(_) => {
			println!("Please only type true or false");
			return login(accounts_dir);
		}
	}
}

pub fn create_account(accounts_dir: &str) -> Result<Keypair, Error> {
	println!("Please create a password");
	let first_password = get_password();
	println!("Please repeat that password");
	if first_password != get_password() {
		println!("Passwords do not match.");
		return create_account(accounts_dir);
	}
	let file_path = [accounts_dir, &get_account_name()].concat();
	let keypair = new_keypair();
	encrypt_and_write(&file_path, &keypair.to_bytes(), &first_password)?;
	Ok(keypair)
}

fn get_account_name() -> String {
	println!("Enter new account name:");
	match text_io::try_read!("{}\n") {
		Ok(i) => i,
		Err(_) => {
			println!("Sorry, couldn't read the input. Try again.");
			get_account_name()
		}
	}
}

fn get_existing_account(accounts_dir: &str) -> Result<Keypair, Error> {
	// Print all accounts
	println!("Accounts:");
	let account_files: Vec<String> = std::fs::read_dir(accounts_dir)
		.map_err(|err| Error::StdIo(err))?
		.filter_map(get_and_print_str)
		.collect();

	// Select & open account
	println!("What account would you like to use?");
	let selection: String = text_io::try_read!().unwrap();
	if account_files.contains(&selection) {
		open_account(selection, accounts_dir)
	} else {
		println!("Invalid selection, please pick an account");
		get_existing_account(accounts_dir)
	}
}

fn open_account(selection: String, accounts_dir: &str) -> Result<Keypair, Error> {
	println!("Please type in the password for {}", selection);
	let password = get_password();
	let full_path = accounts_dir.to_owned() + &selection;
	let file_data = read_and_decrypt(&full_path, &password)?;
	Keypair::from_bytes(&file_data).map_err(|err| Error::SignatureError(err))
}

fn get_and_print_str(input: Result<std::fs::DirEntry, std::io::Error>) -> Option<String> {
	let file = input.ok()?;
	if !file.path().is_file() {
		return None;
	}
	let file_name = file.file_name().to_str()?.to_owned();
	println!("{}", file_name);
	Some(file_name)
}

fn new_keypair() -> Keypair {
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
		Err(_) => {
			println!("Sorry, couldn't read the input. Try again.");
			get_random_from_usr()
		}
	}
}

fn get_password() -> [u8; 32] {
	// A line asking the user to type a password should be printed before this function is called
	let data: String = match text_io::try_read!() {
		Ok(i) => i,
		Err(_) => {
			println!("Sorry, couldn't read the input. Try again.");
			return get_password();
		}
	};

	let mut hasher = Sha256::new();
	hasher.update(data);
	hasher.update("Some extra stuff so it's not just a plain sha256");
	// Note that calling `finalize()` consumes hasher
	let hash = hasher.finalize();
	hash.into()
}
