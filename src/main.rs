extern crate ed25519_dalek;
extern crate text_io;

mod user_keypair;
mod write;
mod read;

fn main() {
	let keypair = user_keypair::get_keypair();
	write::interactive_write(keypair);
	let messages = read::get_messages("test_data/succeed.sml");
	output_messages(messages);
}

fn output_messages(messages: Vec<read::Message>) {
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