use crate::custom_types::*;

pub fn file_type(file_slice: &[u8]) -> Option<SerdeParser> {
	match &file_slice[..2] {
		[146, 147] => Some(SerdeParser::message_pack()),
		[58, 41] => Some(SerdeParser::smile()),
		[123, 10] => Some(SerdeParser::json()),
		[123, 34] => Some(SerdeParser::json()),
		_ => None,
	}
}
