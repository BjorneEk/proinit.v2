

pub mod interface {
	use std::env;
	use colored::Colorize;

	pub fn print_usage() {
		println!(
			" {}: {} <project-name>",
			format!("usage").bold().yellow(),
			program_name()
		);
	}

	pub fn print_warning(warning: String) {
		eprintln!(
			" [{}] {}: {}",
			format!("ERROR").yellow(),
			program_name(),
			warning
		);
	}

	pub fn print_error(error: String) {
		eprintln!(
			" [{}] {}: {}",
			format!("ERROR").bold().red(),
			program_name(),
			error
		);
	}

	pub fn exit_error(error: String, usage: bool) {
		println!("");
		print_error(error);
		if usage { println!(""); print_usage(); }
		println!("");
		std::process::exit(-1);
	}

	fn program_name() -> String {
		match env::current_exe() {
			Ok(name) => match name.file_name() {
				Some(osstr) => match osstr.to_str() {
					Some(str) => String::from(str),
					None      => String::from("")
				},
				None        => String::from("")
			},
			Err(e)   => String::from("")
		}
	}
}
