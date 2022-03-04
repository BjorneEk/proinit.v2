

pub mod interface {
	use std::env;
	use colored::Colorize;

	pub fn print_usage() {
		println!(
			"{}: {} <{}> <{}>",
			format!("usage").bold().yellow(),
			program_name(),
			format!("project-name").yellow(),
			format!("options").yellow()
		);
		println!(
			"{}: {} <{}/{}/{}> <{}>",
			format!("usage").bold().yellow(),
			program_name(),
			format!("-lt").yellow(),
			format!("-ll").yellow(),
			format!("-lb").yellow(),
			format!("language").yellow(),
		);
	}

	pub fn list_languages() {
		println!("{}", format!("Suported languages").bold());
		println!("   {}        -l c", format!("c").bold().bright_blue());
		println!("   {}      -l cpp/c++", format!("c++").bold().bright_blue());
		println!("   {}    -l scala", format!("Scala").bold().bright_blue());
		println!("   {}     -l java", format!("Java").bold().bright_blue());
		println!("   {}     -l rust", format!("Rust").bold().bright_blue());
		println!("   {}     -l bash/shell/shellscript", format!("Bash").bold().bright_blue());
		println!("   {}       -l r/rscript", format!("R").bold().bright_blue());
		println!("   {}  -l inherit, default value", format!("Inherit").bold().bright_blue());
		println!("if no language is specified it will be set to inherit, proinit will select a language based on other settings\n");
	}

	pub fn list_buildtools() {
		println!("{}", format!("Suported buildtools").bold());
		println!("   {}  -l make/makefile", format!("Makefile").bold().bright_blue());
		println!("   {}  -l sbt", format!("SBT").bold().bright_blue());
		println!("   {}  -l cargo", format!("Cargo").bold().bright_blue());
		println!("   {}  -l bash/shell/shellscript", format!("Bash").bold().bright_blue());
		println!("   {}  -l none/nobuildtool", format!("None").bold().bright_blue());
		println!("   {}  -l inherit, defualt value", format!("Inherit").bold().bright_blue());
		println!("if no tool is specified it will be set to inherit, proinit will select a tool based on other settings");
		println!("to speciy a build tool: -bt <buildtool> <template-name>\n");
	}
	pub fn print_help() {
		println!("");
		println!("{}", format!("PROINIT project initializer\n").bold());

		print_usage();
		println!("");
		println!("{}", format!("Options").bold());
		println!("    [{}]  | [{}]                                usage help",
			format!("-h").bold().yellow(),
			format!("--help").bold().yellow()
		);
		println!("    [{}]  | [{}] <{}>                 specify language, try -ll for more help",
			format!("-l").bold().yellow(),
			format!("--language").bold().yellow(),
			format!("language").yellow()
		);
		println!("    [{}] | [{}] <{}> <{}>  specify build tool, try -lb for more help",
			format!("-bt").bold().yellow(),
			format!("--build-tool").bold().yellow(),
			format!("build-tool").yellow(),
			format!("template").yellow()
		);
		println!("    [{}]  | [{}] <{}>                 specify template, try -lt for more help",
			format!("-t").bold().yellow(),
			format!("--template").bold().yellow(),
			format!("template").yellow()
		);
		println!("    [{}] | [{}]                      list available languages",
			format!("-ll").bold().yellow(),
			format!("--list-languages").bold().yellow()
		);
		println!("    [{}] | [{}]                    list available build tools",
			format!("-lb").bold().yellow(),
			format!("--list-build-tools").bold().yellow()
		);
		println!("    [{}] | [{}] <{}>           list available templates",
			format!("-lt").bold().yellow(),
			format!("--list-templates").bold().yellow(),
			format!("language").yellow()
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
