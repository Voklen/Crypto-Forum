use crate::custom_types::SerdeParser;

pub fn file_type(file_slice: &Vec<u8>) -> Option<SerdeParser> {
	match &file_slice[..2] {
		[58, 41] => Some(SerdeParser::Smile),
		[123, 10] => Some(SerdeParser::Json),
		[123, 34] => Some(SerdeParser::Json),
		_ => None,
	}
}
