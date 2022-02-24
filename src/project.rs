


mod project {
	use std::collections::HashMap;

	pub mod flags {
		pub const LANG1:       String  = String::from("-l");
		pub const LANG2:       String  = String::from("--language");
		pub const TEMPLATE1:   String  = String::from("-t");
		pub const TEMPLATE2:   String  = String::from("--template");
		pub const BUILD1:      String  = String::from("-bt");
		pub const BUILD2:      String  = String::from("--build-tool");
		pub const HELP1:       String  = String::from("-h");
		pub const HELP2:       String  = String::from("--HELP");
		pub const TMPLT_INFO1: String  = String::from("-lt");
		pub const TMPLT_INFO2: String  = String::from("--list-templates");
		pub const GIT_SRC1:    String  = String::from("-gs");
		pub const GIT_SRC2:    String  = String::from("--git-source");
		pub const GIT1:        String  = String::from("-g");
		pub const GIT2:        String  = String::from("--git");

		pub const MAKE:        String = String::from("make");
		pub const MAKEFILE:    String = String::from("makefile");
		pub const CARGO:       String = String::from("cargo");
		pub const SBT:         String = String::from("sbt");
		pub const SHELL:       String = String::from("shell");
		pub const BASH:        String = String::from("bash");
		pub const SHELLSCRIPT: String = String::from("shellscript");
		pub const NOBT1:       String = String::from("none");
		pub const NOBT2:       String = String::from("nobuildtool");
		pub const INHERIT:     String = String::from("inherit");

	}

	pub enum Language {
		C,
		Cpp,
		Scala,
		Java,
		Rust,
		Bash,
		R,
		Unspecified,
		Inherit,
	}

	pub enum Buildtool {
		Makefile(String),
		Cargo(String),
		Sbt(String),
		Bash(String),
		NoBuildtool,
		Inherit,
	}

	pub struct Project {
		name       : String,
		language   : Language,
		buildtool : Buildtool,
		template   : String
	}

	impl Clone for Buildtool {
		fn clone(&self) -> Buildtool {
			*self
		}
	}
	impl Clone for Language {
		fn clone(&self) -> Language {
			*self
		}
	}

	impl Language {

		fn from(s: String) -> Language {
			let lang_map: HashMap<&str, Language> = [
				("c",     Language::C),
				("cpp",   Language::Cpp),
				("c++",   Language::Cpp),
				("java",  Language::Java),
				("rust",  Language::Rust),
				("bash",  Language::Bash),
				("shell", Language::Bash),
				("shellscript", Language::Bash),
				("r", Language::R),
				("rscript", Language::R),
				("inherit", Language::Inherit)
				].iter().cloned().collect();
			s.make_ascii_lowercase();
			if lang_map.contains_key(s.as_str()) {
				return match lang_map.get(s.as_str()) {
				Some(lang) => lang.clone(),
				None       => Language::Inherit
			}}
			else { return Language::Inherit; }
		}
	}

	impl Buildtool {
		fn from(args: Vec<String>) -> Buildtool {
			if args.len() < 1 { return Buildtool::Inherit }
			if args.len() < 2 { args.push(String::from("inherit")) }
			match args[0] {
				flags::MAKE  | flags::MAKEFILE                  => Buildtool::Makefile(args[1]),
				flags::CARGO                                    => Buildtool::Cargo(args[1]),
				flags::SBT                                      => Buildtool::Sbt(args[1]),
				flags::SHELL | flags::BASH | flags::SHELLSCRIPT => Buildtool::Bash(args[1]),
				flags::NOBT1 | flags::NOBT2                     => Buildtool::NoBuildtool,
				flags::INHERIT                                  => Buildtool::Inherit
			}
		}
	}

	impl Project {
		fn new(project_name: String, flags: HashMap<String, Vec<String>>) -> Project {
			let mut res = Project {
				name: project_name,
				language:  Language::Inherit,
				buildtool: Buildtool::Inherit,
				template: String::from("")
			};

			for (key, val) in flags.iter() { match key.clone() {
				flags::LANG1     | flags::LANG2     => { res.language  = Language::from(val[0].clone()) },
				flags::BUILD1    | flags::BUILD2    => { res.buildtool = Buildtool::from(val.clone())   },
				flags::TEMPLATE1 | flags::TEMPLATE2 => { res.template  = val[0]                 },
			}}
			return res;
		}
	}
}
