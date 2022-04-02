#[path = "CLI/user_keypair.rs"]
mod user_keypair;
#[path = "CLI/write.rs"]
mod write;

#[path = "backend/read_serde.rs"]
mod read_serde;
#[path = "backend/write_serde.rs"]
mod write_serde;

#[path = "backend/read.rs"]
mod read;

#[derive(Debug)]
pub enum Error {
	StdIo(std::io::ErrorKind),
	SmileError,
	JsonError(serde_json::Error),
}

#[derive(Debug)]
pub struct Message {
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
}

#[derive(Debug)]
pub struct SignatureMessage {
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

#[derive(Debug)]
pub enum SerdeParser {
	Json,
	Smile,
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let messages_file: &str = &args[1];
	let (file_slice, parser) = match read::read_file_data(messages_file) {
		Ok(i) => i,
		Err(Error::StdIo(std::io::ErrorKind::NotFound)) => write::make_file(messages_file).unwrap(),
		_ => std::panic!("error")
	};

	let messages = read_serde::get_messages(&file_slice, &parser).unwrap();
	output_messages(messages);
	
	let keypair = user_keypair::get_keypair();
	write::interactive_write(messages_file, &parser, keypair);
}

fn output_messages(messages: Vec<Message>) {
	for i in messages {
		println!("--------");
		if !i.signed {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		println!("Public key: {:?}", i.public_key.as_bytes());
		println!("Message: \n{}", i.message);
		println!("--------")
	}
}
