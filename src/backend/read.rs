use crate::{Error, SerdeParser};

pub fn read_file_data(file: &str) -> Result<(Vec<u8>, SerdeParser), Error> {
	let file_slice = std::fs::read(file)
		.map_err(|err| Error::StdIo(err))?;
	let parser = file_type(&file_slice);
	Ok((file_slice, parser))
}

fn file_type(file_slice: &Vec<u8>) -> SerdeParser {
	match &file_slice[..2] {
		[58, 41] => SerdeParser::Smile,
		[91, 91] => SerdeParser::Json,
		_ => {
			println!("Unknown file type");
			std::process::exit(1)
		}
	}
}
