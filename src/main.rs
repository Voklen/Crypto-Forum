#[path = "CLI/user_keypair.rs"]
mod user_keypair;
#[path = "CLI/write.rs"]
mod write;

#[path = "backend/read_serde.rs"]
mod read_serde;
#[path = "backend/general_functions.rs"]
mod useful_funcs;
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
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
	pub hash: [u8; 64],
}

#[derive(Debug)]
pub struct SignatureMessage {
	pub prev_hash: [u8; 64],
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signature: ed25519_dalek::Signature,
}

impl SignatureMessage {
	fn get_hash(&self) -> [u8; 64] {
		let mut collection_vector = Vec::<u8>::new();
		collection_vector.extend_from_slice(&self.prev_hash);
		collection_vector.extend_from_slice(&self.public_key.to_bytes());
		collection_vector.extend_from_slice(self.message.as_bytes());
		collection_vector.extend_from_slice(&self.signature.to_bytes());
	
		useful_funcs::hash(&collection_vector)
	}
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
		_ => std::panic!("error"),
	};

	let messages = read_serde::get_messages(&file_slice, &parser).unwrap();
	output_messages(&messages);

	let keypair = user_keypair::get_keypair();
	let last_hash = match messages.last() {
		Some(i) => i.hash,
		None => [0; 64]
	};
	write::interactive_write(messages_file, &parser, keypair, last_hash);
}

fn output_messages(messages: &Vec<Message>) {
	for i in messages {
		println!("--------");
		if !i.signed {
			println!("!!!WARNING: INVALID SIGNATURE!!!");
			println!("!!!WE HAVE NO PROOF THIS PUBLIC KEY EVER POSTED THIS!!!");
		}
		println!("Public key: {}", bytes_to_hex(i.public_key.as_bytes()));
		println!("Replying to message with hash: {}", bytes_to_hex(&i.prev_hash));
		println!("Message: \n{}", i.message);
		println!("Hash: {}", bytes_to_hex(&i.hash));
		println!("--------")
	}
}

fn bytes_to_hex(bytes: &[u8]) -> String {
	let mut out = String::new();
    for i in bytes {
        fn hex_from_digit(num: u8) -> char {
            if num < 10 {
                (b'0' + num) as char
            } else {
                (b'A' + num - 10) as char
            }
        }
		/*
		Amazing, goes from 0 -> 00:
		println!("1: {}", hex_from_digit(0 / 16));
		println!("2: {}", hex_from_digit(0 % 16));
		all the way to the u8 limit of 255 -> FF (Just like colours! I'm getting way to exited about this…):
		println!("1: {}", hex_from_digit(255 / 16));
		println!("2: {}", hex_from_digit(255 % 16));
		*/
        out.push(hex_from_digit(i / 16));
        out.push(hex_from_digit(i % 16));
    }
	out
}