use crate::{
	ask_for_bool,
	encrypt_decrypt::{encrypt_and_write, read_and_decrypt},
	input, Error,
};
use ed25519_dalek::*;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

pub fn login(accounts_dir: &str) -> Result<Keypair, Error> {
	if dir_is_empty(accounts_dir) {
		return create_account(accounts_dir);
	}

	let create_new_account = ask_for_bool("Would you like to create a new account?");
	if create_new_account {
		create_account(accounts_dir)
	} else {
		get_existing_account(accounts_dir)
	}
}

fn dir_is_empty(directory: &str) -> bool {
	match fs::read_dir(directory) {
		Ok(mut files) => {
			if files.next().is_none() {
				true
			} else {
				false
			}
		}
		Err(_) => false,
	}
}

pub fn create_account(accounts_dir: &str) -> Result<Keypair, Error> {
	let account_name = input("Enter new account name:");

	let first_password = get_password("Please create a password");
	let second_password = get_password("Please repeat that password");
	if first_password != second_password {
		println!("Passwords do not match.");
		return create_account(accounts_dir);
	}

	let file_path = [accounts_dir, &account_name].concat();
	let keypair = new_keypair();
	create_dir(accounts_dir)?;
	encrypt_and_write(&file_path, &keypair.to_bytes(), &first_password)?;
	Ok(keypair)
}

fn create_dir(accounts_dir: &str) -> Result<(), Error> {
	fs::create_dir(accounts_dir).or_else(|err| {
		if err.kind() == std::io::ErrorKind::AlreadyExists {
			Ok(())
		} else {
			Err(Error::StdIo(err))
		}
	})
}

fn get_existing_account(accounts_dir: &str) -> Result<Keypair, Error> {
	println!("Accounts:");
	let account_files = get_and_print_accounts(accounts_dir)?;

	// Select & open account
	let selection = input("What account would you like to use?");
	if account_files.contains(&selection) {
		open_account(selection, accounts_dir)
	} else {
		println!("Invalid selection, please pick an account");
		get_existing_account(accounts_dir)
	}
}

fn open_account(selection: String, accounts_dir: &str) -> Result<Keypair, Error> {
	let password = get_password(&format!("Please enter the password for {}", selection));
	let full_path = accounts_dir.to_owned() + &selection;
	let file_data = read_and_decrypt(&full_path, &password)?;
	Keypair::from_bytes(&file_data).map_err(Error::SignatureError)
}

fn get_and_print_accounts(accounts_dir: &str) -> Result<Vec<String>, Error> {
	let account_files: Vec<String> = fs::read_dir(accounts_dir)
		.map_err(Error::StdIo)?
		.filter_map(get_and_print_str)
		.collect();
	Ok(account_files)
}

fn get_and_print_str(input: Result<fs::DirEntry, std::io::Error>) -> Option<String> {
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
	Keypair { secret, public }
}

/// Get the user to enter some random characters, then hash whatever they give and return that hash as a byte array
fn get_random_from_usr() -> [u8; 64] {
	let random_input = input(
		"Please type some random characters (this will be used for the initial key generation)",
	);
	let hash = Sha512::digest(random_input);
	hash.into()
}

fn get_password(prompt: &str) -> [u8; 32] {
	let user_input = input(prompt);
	let hash = Sha256::digest(user_input);
	hash.into()
}
