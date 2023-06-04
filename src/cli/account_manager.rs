use crate::{
	encrypt_decrypt::{encrypt_and_write, read_and_decrypt},
	errors::throw,
	input::*,
	Error,
};
use ed25519_dalek::*;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

pub fn login(accounts_dir: &str) -> Keypair {
	create_dir(accounts_dir);
	if dir_is_empty(accounts_dir) {
		return create_account(accounts_dir);
	}
	get_existing_account(accounts_dir)
}

fn dir_is_empty(directory: &str) -> bool {
	match fs::read_dir(directory) {
		Ok(mut files) => files.next().is_none(),
		//TODO better handle errors
		Err(_) => false,
	}
}

fn create_account(accounts_dir: &str) -> Keypair {
	let account_name = input("Enter new account name:");

	let first_password = get_password("Please create a password");
	let second_password = get_password("Please repeat that password");
	if first_password != second_password {
		println!("Passwords do not match.");
		return create_account(accounts_dir);
	}

	let file_path = [accounts_dir, &account_name].concat();
	let keypair = new_keypair();
	encrypt_and_write(&file_path, &keypair.to_bytes(), &first_password).unwrap();
	keypair
}

fn create_dir(accounts_dir: &str) {
	let result = fs::create_dir(accounts_dir);
	let err = match result {
		Ok(()) => return,
		Err(err) => err,
	};
	if err.kind() == std::io::ErrorKind::AlreadyExists {
		return;
	}
	let err_message = format!("Error creating directory: {err}");
	throw(&err_message);
}

fn get_existing_account(accounts_dir: &str) -> Keypair {
	println!("Accounts:");
	let account_files = get_and_print_accounts(accounts_dir);

	let prompt = "What account would you like to use? (type \"new\" to create a new one)";
	let selection = input(prompt);
	if account_files.contains(&selection) {
		return open_account(&selection, accounts_dir).unwrap();
	}
	if &selection == "new" {
		return create_account(accounts_dir);
	}
	println!("Invalid selection, please pick an account");
	get_existing_account(accounts_dir)
}

fn open_account(selection: &str, accounts_dir: &str) -> Result<Keypair, Error> {
	let password = get_password(&format!("Please enter the password for {selection}"));
	let full_path = accounts_dir.to_owned() + &selection;
	let file_data = read_and_decrypt(&full_path, &password)?;
	Keypair::from_bytes(&file_data).map_err(Error::SignatureError)
}

fn get_and_print_accounts(accounts_dir: &str) -> Vec<String> {
	let files = get_account_files(accounts_dir);
	files.filter_map(get_and_print_str).collect()
}

fn get_account_files(accounts_dir: &str) -> fs::ReadDir {
	match fs::read_dir(accounts_dir) {
		Ok(res) => res,
		Err(e) => throw(&format!(
			"Failed to retrieve accounts from {accounts_dir}: {e}"
		)),
	}
}

fn get_and_print_str(input: Result<fs::DirEntry, std::io::Error>) -> Option<String> {
	let file = input.ok()?;
	if !file.path().is_file() {
		return None;
	}
	let file_name = file.file_name().to_str()?.to_owned();
	println!("{file_name}");
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
