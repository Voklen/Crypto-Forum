use crate::custom_types::{Error, SerdeParser};

pub fn file_type(file_slice: &Vec<u8>) -> Option<SerdeParser> {
	match &file_slice[..2] {
		[58, 41] => Some(SerdeParser::Smile),
		[91, 91] => Some(SerdeParser::Json),
		_ => None
	}
}
