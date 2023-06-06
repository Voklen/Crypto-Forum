use crypto_forum::custom_types::*;

pub fn get_args() -> (Vec<String>, Vec<Argument>) {
	// Skip the first argument because it's just the executable path
	let arguments_as_strings: Vec<String> = std::env::args().skip(1).collect();

	if arguments_as_strings.is_empty() {
		println!(
			"{program_name}: missing operand",
			program_name = env!("CARGO_PKG_NAME")
		);
		std::process::exit(1)
	}

	let mut links = Vec::<String>::new();
	let mut arguments = Vec::<Argument>::new();
	for arg in arguments_as_strings {
		if arg.starts_with('-') {
			arguments.push(parse_dashes(&arg))
		} else {
			// If there is no "-" at the start of the argument, it's a link that's being passed
			links.push(arg);
		}
	}
	(links, arguments)
}

fn parse_dashes(arg: &str) -> Argument {
	match arg {
		"-i" => Argument::Interactive,
		"-m" => Argument::MachineOutput,
		"-c" => Argument::Create,
		"-v" => print_version_info(),
		"-h" => print_help(),

		"--interactive" => Argument::Interactive,
		"--machine-output" => Argument::MachineOutput,
		"--create" => Argument::Create,
		"--version" => print_version_info(),
		"--help" => print_help(),

		_ => unknown_arg(arg),
	}
}

fn unknown_arg(arg: &str) -> Argument {
	let program_name = env!("CARGO_PKG_NAME");
	println!("{program_name}: invalid option -- '{arg}'",);
	std::process::exit(1)
}

fn print_version_info() -> Argument {
	println!(
		"{program_name} {program_ver}",
		program_name = env!("CARGO_PKG_NAME"),
		program_ver = env!("CARGO_PKG_VERSION")
	);
	println!(
		"
Copyright (C) 2022 Alexander Gorichev
License GPL-3.0-only: GNU GPL version 3.0 only <https://gnu.org/licenses/gpl-3.0.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Alexander Gorichev.
Send all issues to <https://github.com/Voklen/Crypto-Forum/issues> or <Alex.Gorichev@protonmail.com>
I hope you enjoy :)"
	);
	std::process::exit(0)
}

fn print_help() -> ! {
	let program_name = env!("CARGO_PKG_NAME");
	println!(
		"
Usage: {program_name} [OPTION]...
Distributed git service

	-i  --interactive     run in interactive mode
	-m  --machine-output  print in a way more sutible for parsing
	-c  --create          create a new repo (no link required as argument)
	-v  --version         output version information and exit
	-h  --help            display this help and exit

To create a new repo run:
	{program_name} -c
To edit an existing repo use:
	{program_name} -i <IPNS link>

Send all issues to <https://github.com/Voklen/Crypto-Forum/issues> or <Alex.Gorichev@protonmail.com>
"
	);
	std::process::exit(0)
}
