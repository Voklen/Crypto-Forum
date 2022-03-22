use std::panic;

use crate::{Error, SerdeParser};

pub fn read_file_data(file: &str) -> Result<(SerdeParser, Vec<u8>), Error> {
	let file_slice = read_file_to_slice(file)?;
	let parser = file_type(&file_slice);
	return Ok((parser, file_slice));
}

fn read_file_to_slice(file: &str) -> Result<Vec<u8>, Error> {
	use std::io::Read;

	let file = match std::fs::File::open(file) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(i) => i,
	};

	let mut file_slice = Vec::<u8>::new();
	match (&file).read_to_end(&mut file_slice) {
		Err(i) => return Err(Error::StdIo(i.kind())),
		Ok(_) => {}
	};

	Ok(file_slice)
}

fn file_type(file_slice: &Vec<u8>) -> SerdeParser {
	match &file_slice[..2] {
		[58, 41] => SerdeParser::Smile,
		[91, 91] => SerdeParser::Json,
		i => panic!("{:?}", i)
	}
}
