fn encode(input: [u8; 64]) -> [u8; 86] {
	const SIZE: usize = 64;
	const CHUNKS: usize = SIZE / 3;
	const REMAINDER: usize = SIZE % 3;
	const LENGTH: usize = match REMAINDER {
		0 => CHUNKS * 4,
		r => (CHUNKS * 4) + r + 1,
	};
	let mut output = [0u8; LENGTH];

	let mut iter = input.into_iter();
	for i in 0..CHUNKS {
		let index = i * 4;
		let end_index = index + 4;

		let chunk = [
			iter.next().unwrap(),
			iter.next().unwrap(),
			iter.next().unwrap(),
		];
		output[index..end_index].swap_with_slice(&mut split::three(chunk))
	}
	match REMAINDER {
		0 => {}
		1 => output[LENGTH - 2..].swap_with_slice(&mut split::one(iter.next().unwrap())),
		2 => output[LENGTH - 3..].swap_with_slice(&mut split::two([
			iter.next().unwrap(),
			iter.next().unwrap(),
		])),
		_ => unreachable!(),
	};

	let mut converted = [0u8; 86];
	for i in 0..86 {
		converted[i] = encode_table[output[i] as usize];
	}

	return converted;
}
mod split {
	pub fn one(chunk: u8) -> [u8; 2] {
		[&chunk >> 2, (&chunk & 0b00000011) << 4]
	}

	pub fn two(chunk: [u8; 2]) -> [u8; 3] {
		[
			&chunk[0] >> 2,
			(&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
			(&chunk[1] & 0b00001111) << 2,
		]
	}

	pub fn three(chunk: [u8; 3]) -> [u8; 4] {
		[
			&chunk[0] >> 2,
			(&chunk[0] & 0b00000011) << 4 | &chunk[1] >> 4,
			(&chunk[1] & 0b00001111) << 2 | &chunk[2] >> 6,
			&chunk[2] & 0b00111111,
		]
	}
}

const fn generate_alphabet() -> [u8; 64] {
	let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
	let mut symbols = [0_u8; 64];
	let source_bytes = alphabet.as_bytes();

	// a way to copy that's allowed in const fn
	let mut index = 0;
	while index < 64 {
		symbols[index] = source_bytes[index];
		index += 1;
	}

	symbols
}

const encode_table: [u8; 64] = generate_alphabet();

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

/// Convert hex string to bytes of a generic array size
/// ```
/// use crypto_forum::base64::hex_to_bytes;
///
/// let result = hex_to_bytes("DF36");
/// assert_eq!(result, Some([223, 54]))
/// ```
/// If there are not enough characters for the array size, it will return None
/// ```
/// use crypto_forum::base64::hex_to_bytes;
///
/// let hex = "FFFFFFFF";
/// let result: Option<[u8; 49]> = hex_to_bytes(&hex);
/// assert_eq!(result, None)
/// ```
/// If the string has out of range characters None will also be returned
/// ```
/// use crypto_forum::base64::hex_to_bytes;
///
/// let hex = "x";
/// let result: Option<[u8; 64]> = hex_to_bytes(&hex);
/// assert_eq!(result, None)
/// ```
pub fn hex_to_bytes<const COUNT: usize>(hex_string: &str) -> Option<[u8; COUNT]> {
	fn digit_from_hex(character: char) -> u8 {
		let num = character as u8;
		if num < (b'0' + 10) {
			num - b'0'
		} else {
			num - b'A' + 10
		}
	}
	let mut chars = hex_string.chars();
	let mut bytes = [0; COUNT];
	for i in 0..COUNT {
		let sixteens_digit = chars.next()?;
		let ones_digit = chars.next()?;

		let byte = digit_from_hex(sixteens_digit).checked_mul(16)?;
		let second_byte = digit_from_hex(ones_digit);

		bytes[i] = byte + second_byte
	}
	Some(bytes)
}

/// Convert hex string to bytes of a generic array size
/// ```
/// use crypto_forum::base64::unchecked_hex_to_bytes;
///
/// let result = unchecked_hex_to_bytes("DF36");
/// assert_eq!(result, Some([223, 54]))
/// ```
/// If there are not enough characters for the array size, it will return None
/// ```
/// use crypto_forum::base64::unchecked_hex_to_bytes;
///
/// let hex = "FFFFFFFF";
/// let result: Option<[u8; 49]> = unchecked_hex_to_bytes(&hex);
/// assert_eq!(result, None)
/// ```
/// # Panics
/// If the string has out of range characters a panic will be triggered, for example unchecked_hex_to_bytes("x")
pub fn unchecked_hex_to_bytes<const COUNT: usize>(hex_string: &str) -> Option<[u8; COUNT]> {
	fn digit_from_hex(character: char) -> u8 {
		let num = character as u8;
		if num < (b'0' + 10) {
			num - b'0'
		} else {
			num - b'A' + 10
		}
	}
	let mut chars = hex_string.chars();
	let mut bytes = [0; COUNT];
	for i in 0..COUNT {
		let sixteens_digit = chars.next()?;
		let ones_digit = chars.next()?;

		let byte = digit_from_hex(sixteens_digit) * 16;
		let second_byte = digit_from_hex(ones_digit);

		bytes[i] = byte + second_byte
	}
	Some(bytes)
}

#[test]
fn large() {
	let input = [
		20, 154, 181, 216, 186, 90, 229, 10, 255, 138, 145, 207, 92, 84, 206, 254, 16, 217, 187,
		148, 132, 179, 179, 25, 176, 236, 66, 222, 40, 146, 222, 22, 175, 203, 233, 19, 127, 170,
		84, 201, 56, 66, 135, 60, 239, 125, 78, 207, 14, 246, 92, 122, 23, 36, 54, 80, 180, 43,
		213, 89, 127, 6, 139, 116, 43, 96, 11, 74, 243, 95, 247, 45, 167, 20, 137, 38, 242, 206,
		228, 116, 206, 106, 113, 48, 220, 157, 116, 63, 219, 7, 57, 60, 53, 28, 225, 235, 108, 134,
		194, 210, 117, 242, 237, 103, 145, 47, 95, 99, 155, 189, 219, 103, 131, 154, 141, 108, 41,
		250, 214, 51, 245, 29, 22, 88, 140, 85, 173, 63, 192, 192, 21, 12, 246, 170, 126, 127, 228,
		187, 24, 253, 56, 137, 148, 22, 100, 10, 62, 166, 97, 101, 145, 171, 57, 200, 232, 79, 241,
		149, 184, 103, 140, 122, 35, 251, 30, 226, 137, 200, 62, 158, 48, 46, 178, 177, 24, 72,
		238, 94, 165, 179, 187, 169, 56, 87, 82, 126, 65, 24, 126, 74, 58, 201, 107, 150, 29, 25,
		8, 53, 65, 21, 193, 248, 234, 185, 171, 20, 93, 84, 7, 188, 97, 141, 185, 88, 42, 253, 174,
		216, 74, 157, 62, 238, 230, 156, 211, 131, 152, 177, 139, 6, 241, 136, 191, 59, 115, 198,
		55, 103, 210, 69, 198, 123, 82, 150, 230, 183, 42, 17, 99, 140, 226, 15, 155, 245, 27, 141,
		33, 188, 30, 98, 34, 170, 159, 182, 210, 37, 110, 102, 145, 182, 242, 248, 144, 254, 17,
		63, 79, 192, 43, 154, 77, 192, 233, 5, 220, 224, 215, 151, 117, 47, 240, 169, 228, 117, 69,
		163, 248, 207, 193, 242, 239, 56, 135, 16, 156, 208, 42, 191, 236, 107, 221, 156, 94, 93,
		173, 195, 112, 174, 104, 87, 138, 167, 241, 226, 120, 57, 113, 73, 151, 44, 228, 45, 18,
		242, 72, 23, 131, 73, 11, 202, 238, 82, 235, 52, 124, 171, 222, 39, 92, 7, 119, 185, 215,
		49, 245, 192, 52, 195, 117, 187, 13, 30, 56, 184, 234, 48, 171, 138, 218, 124, 33, 47, 203,
		142, 189, 2, 19, 243, 195, 158, 87, 131, 12, 37, 151, 143, 40, 66, 79, 64, 160, 113, 128,
		83, 254, 133, 199, 78, 238, 178, 24, 2, 224, 63, 148, 108, 222, 213, 27, 7, 122, 17, 146,
		199, 124, 231, 159, 88, 161, 251,
	];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes(&hex).unwrap();
	assert_eq!(input, result);
}

#[test]
fn all_zeros() {
	let input = [0; 64];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes(&hex).unwrap();
	assert_eq!(input, result);
}

#[test]
fn all_255() {
	let input = [255; 64];
	let hex = bytes_to_hex(&input);
	let result = hex_to_bytes(&hex).unwrap();
	assert_eq!(input, result);
}

#[test]
#[should_panic]
fn non_hex_string() {
	let hex = "XX";
	let _: Option<[u8; 64]> = unchecked_hex_to_bytes(&hex);
}

#[test]
fn too_small_array() {
	let hex = "FFFFFFFF";
	let _: [u8; 2] = hex_to_bytes(&hex).unwrap();
}

#[test]
fn too_large_array() {
	let hex = "FFFFFFFF";
	let result: Option<[u8; 69]> = hex_to_bytes(&hex);
	assert_eq!(result, None)
}

#[test]
fn all_255_64() {
	let input = [
		0x4e, 0x18, 0xac, 0xfa, 0x2b, 0x3e, 0x6d, 0xab, 0x1e, 0xfb, 0xae, 0x3e, 0xc2, 0x8a, 0xe4,
		0x8a, 0x78, 0x3e, 0xbd, 0xea, 0xf2, 0xfb, 0x07, 0xa5, 0x94, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	];
	let converted = encode(input);
	let string = std::str::from_utf8(&converted).unwrap();

	assert_eq!(
		"This-is-base-64-working-very-wellAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
		string
	);
}
