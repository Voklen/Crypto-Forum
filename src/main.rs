#[path ="CLI/user_keypair.rs"]
mod user_keypair;
#[path ="CLI/write.rs"]
mod write;

#[path ="backend/write_smile.rs"]
mod write_smile;
#[path ="backend/read.rs"]
mod read;

fn main() {
	let messages_file = "messages.sml";
	
	let keypair = user_keypair::get_keypair();
	write::interactive_write(messages_file, keypair);
	let messages = read::get_messages(messages_file).unwrap();
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