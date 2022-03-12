pub fn write_to_smile(file: &str, data: Vec<([u8; 32], String, [u8; 32], [u8; 32])>) {
	use std::io::Write;

	let orig_messages = crate::read_smile::get_messages_vec(file)
		.unwrap_or(Vec::new());
	let final_write_data = [orig_messages, data].concat();
	let value = serde_smile::to_vec(&final_write_data).unwrap();

	let mut file = std::fs::File::create(file).unwrap();
	file.write_all(&value);
}

/* Hope to eventually replace this with a better way, probably including slices */
pub fn to_32(input: [u8; 64], first_32: bool) -> [u8; 32] {
	let offset = if first_32 { 0 } else { 32 };
	let mut out = [0; 32];
	for (i, element) in input.iter().enumerate() {
		if offset <= i && i < (32 + offset) {
			out[i - offset] = *element;
		}
	}
	out
}