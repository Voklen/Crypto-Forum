extern crate ed25519_dalek;
extern crate text_io;

mod user_keypair;
mod write;
mod read;

fn main() {
	let keypair = user_keypair::get_keypair();
	write::interactive_write(keypair);
	let messages = read::get_messages("test_data/succeed.sml");
	println!("Messages: {:?}", messages)
}