pub fn throw(error_message: &str) -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: {error_message}");
	#[cfg(not(debug_assertions))]
	std::process::exit(1);
	#[cfg(debug_assertions)]
	panic!();
}
