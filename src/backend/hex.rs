pub fn bytes_to_hex(bytes: &[u8]) -> String {
	let mut hex_string = String::new();
	for i in bytes {
		fn hex_from_digit(num: u8) -> char {
			if num < 10 {
				(b'0' + num) as char
			} else {
				(b'A' + num - 10) as char
			}
		}
		/*
		Amazing, goes from 0 -> 00:
		println!("1: {}", hex_from_digit(0 / 16));
		println!("2: {}", hex_from_digit(0 % 16));
		all the way to the u8 limit of 255 -> FF (Just like colours! I'm getting way to exited about this…):
		println!("1: {}", hex_from_digit(255 / 16));
		println!("2: {}", hex_from_digit(255 % 16));
		*/
		hex_string.push(hex_from_digit(i / 16));
		hex_string.push(hex_from_digit(i % 16));
	}
	hex_string
}

pub fn hex_to_bytes(hex_string: String) -> Vec<u8> {
	fn digit_from_hex(character: char) -> u8 {
		let num = character as u8;
		if num < (b'0' + 10) {
			num - b'0'
		} else {
			num - b'A' + 10
		}
	}
	let mut chars = hex_string.chars();
	let mut bytes = Vec::<u8>::new();
	for _ in 0..(hex_string.len() / 2) {
		let sixteens_digit = chars.next().unwrap();
		let ones_digit = chars.next().unwrap();

		let byte = digit_from_hex(sixteens_digit) * 16;
		let second_byte = digit_from_hex(ones_digit);

		bytes.push(byte + second_byte)
	}
	bytes
}

pub fn hex_to_bytes64(hex_string: &str) -> [u8; 64] {
	fn digit_from_hex(character: char) -> u8 {
		let num = character as u8;
		if num < (b'0' + 10) {
			num - b'0'
		} else {
			num - b'A' + 10
		}
	}
	let mut chars = hex_string.chars();
	let mut bytes = [0; 64];
	for i in 0..64 {
		let sixteens_digit = chars.next().unwrap();
		let ones_digit = chars.next().unwrap();

		let byte = digit_from_hex(sixteens_digit) * 16;
		let second_byte = digit_from_hex(ones_digit);

		bytes[i] = byte + second_byte
	}
	bytes
}

#[test]
fn normal() {
	let input = [
		223, 54, 208, 218, 182, 40, 43, 67, 126, 144, 234, 122, 188, 1, 244, 145, 178, 155, 128,
		132, 104, 202, 56, 75, 182, 52, 30, 189, 85, 187, 212, 26, 209, 88, 143, 230, 22, 220, 153,
		232, 13, 149, 168, 68, 222, 167, 36, 247, 218, 175, 31, 119, 94, 127, 94, 24, 41, 55, 251,
		106, 73, 36, 252, 6,
	];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes64(&hex);
	assert_eq!(input, result);
}

#[test]
fn all_zeros() {
	let input = [0; 64];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes64(&hex);
	assert_eq!(input, result);
}

#[test]
fn all_255() {
	let input = [255; 64];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes64(&hex);
	assert_eq!(input, result);
}
