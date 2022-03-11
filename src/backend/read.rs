use ed25519_dalek::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    StdIo(std::io::ErrorKind),
    SmileError,
}

#[derive(Debug)]
pub struct Message {
    pub public_key: PublicKey,
    pub message: String,
    pub signed: bool,
}

pub fn get_messages(file: &str) -> Result<Vec<Message>, Error> {
    Ok(
		get_messages_vec(file)?
        .into_iter()
        .map(|x| {
            let public_key = PublicKey::from_bytes(&x.0).unwrap();
            let message = x.1;
            let signature = Signature::from_bytes(&[x.2, x.3].concat()).unwrap(); // Combine the two parts of the signature back into one
            let signed = public_key.verify(message.as_bytes(), &signature).is_ok();
            Message {
                public_key,
                message,
                signed,
            }
        })
        .collect()
	)
}

pub fn get_messages_vec(file: &str) -> Result<Vec<([u8; 32], String, [u8; 32], [u8; 32])>, Error> {
    use std::io::Read;

    let file = match std::fs::File::open(file) {
        Err(i) => return Err(Error::StdIo(i.kind())),
        Ok(i) => i,
    };
    let mut smile = Vec::<u8>::new();

    match (&file).read_to_end(&mut smile) {
        Err(i) => return Err(Error::StdIo(i.kind())),
        Ok(i) => i,
    };

    Ok(match serde_smile::from_slice(&smile) {
        Err(_) => return Err(Error::SmileError), // serde_smile unfortunately does not expose the ErrorKind enum as public so we cannot specify the error
        Ok(i) => i,
    })
}
