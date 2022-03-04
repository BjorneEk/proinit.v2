


pub mod project {
	use std::collections::HashMap;

	#[derive(Copy, Clone)]
	pub enum Language {
		C,
		Cpp,
		Scala,
		Java,
		Rust,
		Bash,
		R,
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
		pub name       : String,
		pub language   : Language,
		pub buildtool : Buildtool,
		pub template   : String
	}


	impl Language {

		pub fn from(s: String) -> Language {
			let lang_map: HashMap<&str, Language> = [
				("c",     Language::C),
				("cpp",   Language::Cpp),
				("c++",   Language::Cpp),
				("scala", Language::Scala),
				("java",  Language::Java),
				("rust",  Language::Rust),
				("bash",  Language::Bash),
				("shell", Language::Bash),
				("shellscript", Language::Bash),
				("r", Language::R),
				("rscript", Language::R),
				("inherit", Language::Inherit)
				].iter().cloned().collect();
			let mut tmp = s.clone();
			tmp.make_ascii_lowercase();
			println!("lang: {}", tmp);
			if lang_map.contains_key(tmp.as_str()) {
				return match lang_map.get(tmp.as_str()) {
				Some(lang) => lang.clone(),
				None       => Language::Inherit
			}}
			else { return Language::Inherit; }
		}

		pub  fn to_string(&self) -> String { match &self {
			Language::C       => String::from("C"),
			Language::Cpp     => String::from("C++"),
			Language::Scala   => String::from("Scala"),
			Language::Java    => String::from("Java"),
			Language::Rust    => String::from("Rust"),
			Language::Bash    => String::from("Bash"),
			Language::R       => String::from("R"),
			Language::Inherit => String::from("Inherit"),
		}}
	}

	impl Buildtool {
		fn from(args: Vec<String>) -> Buildtool {
			if args.len() < 1 { return Buildtool::Inherit }
			let mut tmp = args.clone();
			if tmp.len() < 2 { tmp.push(String::from("inherit")) }
			tmp[0].make_ascii_lowercase();
			match tmp[0].as_str() {
				"make"  | "makefile"             => Buildtool::Makefile(tmp[1].clone()),
				"cargo"                          => Buildtool::Cargo(tmp[1].clone()),
				"sbt"                            => Buildtool::Sbt(tmp[1].clone()),
				"shell" | "bash" | "shellscript" => Buildtool::Bash(tmp[1].clone()),
				"none"  | "nobuildtool"          => Buildtool::NoBuildtool,
				"inherit"                        => Buildtool::Inherit,
				_ => Buildtool::Inherit
			}
		}

		pub fn to_string(&self) -> String { match &self {
			Buildtool::Makefile(tmplt) => "Makefile ".to_owned(),
			Buildtool::Cargo(tmplt)    => "Cargo ".to_owned(),
			Buildtool::Sbt(tmplt)      => "SBT ".to_owned(),
			Buildtool::Bash(tmplt)     => "Bash ".to_owned(),
			Buildtool::NoBuildtool     => "None ".to_owned(),
			Buildtool::Inherit         => "Inherit ".to_owned(),
		}}
	}

	impl Project {
		pub fn from(project_name: String, flags: HashMap<String, Vec<String>>) -> Project {
			let mut res = Project {
				name:      project_name,
				language:  Language::Inherit,
				buildtool: Buildtool::Inherit,
				template:  String::from("default")
			};

			for (key, val) in flags.iter() { match key.as_str() {
				"-l"  | "--language"   => { res.language  = Language::from(val[0].clone()) },
				"-bt" | "--build-tool" => { res.buildtool = Buildtool::from(val.clone())   },
				"-t"  | "--template"   => { res.template  = val[0].clone()                 },
				_ => println!("unknown flag")
			}}
			return res;
		}
	}
}
