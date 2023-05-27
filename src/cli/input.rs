pub fn input(prompt: &str) -> String {
	println!("{prompt}");

	let mut input_string = String::new();
	let read_result = std::io::stdin().read_line(&mut input_string);

	if read_result.is_err() {
		println!("Could not read the input, try again.");
		return input(prompt);
	};
	input_string.trim().into()
}

pub fn ask_for_bool(prompt: &str) -> bool {
	let prompt = &format!("{prompt} (y/n)");
	let input = input(prompt);

	let lowercase: &str = &input.trim().to_lowercase();
	match lowercase {
		"y" => true,
		"yes" => true,
		"true" => true,
		"n" => false,
		"no" => false,
		"false" => false,
		_ => {
			println!("Please type y(es) or n(o)");
			ask_for_bool(prompt)
		}
	}
}
