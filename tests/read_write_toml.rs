use std::path::Path;

use crypto_forum::{custom_types::Message, *};

macro_rules! setup {
	($test_data:ident) => {
		let $test_data = vec![
			Message {
				prev_hash: [
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				],
				public_key: ed25519_dalek::PublicKey::from_bytes(&[
					40, 151, 31, 122, 201, 208, 88, 167, 131, 68, 167, 174, 137, 226, 120, 246, 76,
					47, 160, 112, 83, 73, 250, 195, 91, 192, 40, 220, 255, 12, 254, 77,
				])
				.unwrap(),
				message: "hello".to_string(),
				signature: ed25519_dalek::Signature::from_bytes(&[
					223, 54, 208, 218, 182, 40, 43, 67, 126, 144, 234, 122, 188, 1, 244, 145, 178,
					155, 128, 132, 104, 202, 56, 75, 182, 52, 30, 189, 85, 187, 212, 26, 209, 88,
					143, 230, 22, 220, 153, 232, 13, 149, 168, 68, 222, 167, 36, 247, 218, 175, 31,
					119, 94, 127, 94, 24, 41, 55, 251, 106, 73, 36, 252, 6,
				])
				.unwrap(),
			},
			Message {
				prev_hash: [
					23, 189, 24, 38, 108, 90, 212, 238, 197, 124, 125, 252, 172, 131, 189, 160,
					150, 146, 57, 248, 122, 37, 169, 26, 54, 148, 253, 214, 156, 86, 84, 48, 213,
					126, 169, 110, 112, 193, 139, 171, 86, 28, 176, 254, 96, 7, 144, 162, 111, 100,
					223, 91, 123, 169, 69, 168, 29, 52, 236, 123, 77, 16, 200, 149,
				],
				public_key: ed25519_dalek::PublicKey::from_bytes(&[
					40, 151, 31, 122, 201, 208, 88, 167, 131, 68, 167, 174, 137, 226, 120, 246, 76,
					47, 160, 112, 83, 73, 250, 195, 91, 192, 40, 220, 255, 12, 254, 77,
				])
				.unwrap(),
				message: "I don't like you".to_string(),
				signature: ed25519_dalek::Signature::from_bytes(&[
					55, 22, 73, 33, 67, 132, 9, 19, 178, 138, 174, 109, 102, 150, 129, 250, 127,
					24, 76, 227, 111, 117, 194, 103, 89, 173, 79, 204, 139, 73, 213, 239, 254, 98,
					180, 159, 42, 153, 72, 77, 190, 1, 11, 35, 2, 4, 255, 73, 74, 177, 24, 133,
					116, 230, 132, 214, 7, 129, 64, 237, 240, 195, 164, 15,
				])
				.unwrap(),
			},
			Message {
				prev_hash: [
					149, 74, 32, 90, 143, 242, 25, 201, 57, 137, 140, 242, 192, 170, 44, 201, 164,
					115, 234, 186, 69, 206, 68, 89, 53, 141, 43, 198, 174, 84, 201, 63, 163, 51,
					175, 3, 185, 164, 154, 173, 70, 214, 68, 15, 55, 255, 214, 209, 85, 111, 250,
					212, 90, 187, 35, 54, 234, 131, 210, 235, 79, 68, 113, 148,
				],
				public_key: ed25519_dalek::PublicKey::from_bytes(&[
					253, 99, 221, 70, 160, 245, 208, 129, 25, 20, 153, 153, 64, 186, 254, 162, 142,
					21, 35, 154, 156, 112, 85, 119, 46, 47, 26, 81, 221, 62, 25, 92,
				])
				.unwrap(),
				message: "hi".to_string(),
				signature: ed25519_dalek::Signature::from_bytes(&[
					223, 143, 138, 176, 246, 219, 128, 144, 59, 194, 47, 28, 98, 36, 61, 23, 203,
					248, 98, 88, 7, 44, 78, 64, 126, 239, 100, 117, 112, 77, 177, 196, 241, 25, 2,
					102, 38, 203, 234, 128, 230, 158, 180, 174, 149, 12, 213, 12, 113, 156, 133,
					185, 76, 86, 100, 137, 115, 78, 19, 220, 175, 216, 52, 5,
				])
				.unwrap(),
			},
		];
	};
}

#[test]
fn read() {
	setup!(expected);

	let test_file = "reference/reference.toml";
	let file_slice = std::fs::read_to_string(test_file).unwrap();
	let actual = read_serde::get_messages(&file_slice).unwrap();
	assert_eq!(actual, expected);
}

#[test]
fn read_write() {
	setup!(test_data);
	let expected = test_data.clone();

	const TEST_DIR: &str = "test_data/";
	const TEST_FILE: &str = "read_write.toml";
	let test_path = &format!("{TEST_DIR}{TEST_FILE}");
	if Path::new(test_path).exists() {
		std::fs::remove_file(test_path).unwrap();
	}
	std::fs::create_dir_all(TEST_DIR).unwrap_or_else(dir_error);

	write_serde::write_messages(test_path, test_data).unwrap();
	let file_slice = std::fs::read_to_string(test_path).unwrap();
	let actual = read_serde::get_messages(&file_slice).unwrap();
	assert_eq!(actual, expected);

	std::fs::remove_file(test_path).unwrap();
}

fn dir_error(error: std::io::Error) {
	match error.kind() {
		std::io::ErrorKind::AlreadyExists => {}
		_ => panic!("Error creating test directory: {error}"),
	}
}
