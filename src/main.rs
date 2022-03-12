#[path ="CLI/user_keypair.rs"]
mod user_keypair;
#[path ="CLI/write.rs"]
mod write;

#[path ="backend/write_smile.rs"]
mod write_smile;
#[path ="backend/read_smile.rs"]
mod read_smile;

#[derive(Debug, PartialEq)]
pub enum Error {
	StdIo(std::io::ErrorKind),
	SmileError,
}

#[derive(Debug)]
pub struct Message {
	pub public_key: ed25519_dalek::PublicKey,
	pub message: String,
	pub signed: bool,
}

fn main() {
	let messages_file = "messages.sml";
	
	let keypair = user_keypair::get_keypair();
	write::interactive_write(messages_file, keypair);
	let messages = read_smile::get_messages(messages_file).unwrap();
	output_messages(messages);
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