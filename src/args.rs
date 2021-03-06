



pub mod args {
	use std::env;
	use std::collections::HashMap;



	pub fn project_name() -> String {
		match env::args().nth(1) {
			Some(name) => name,
			None       => String::from("-")
		}
	}

	pub fn get_args() -> Result<HashMap<String, Vec<String>>, &'static str> {
		return format_args(env::args().collect());
	}

	fn format_args(args_in: Vec<String>) -> Result<HashMap<String, Vec<String>>, &'static str> {

		let mut res = HashMap::new();

		for (i, arg) in args_in.iter().enumerate() {

			if arg.chars().next().unwrap() == '-' && i < args_in.len() {

				let mut params = Vec::new();
				if res.contains_key(arg) { return Err("argument declared twice"); }
				for param in &mut args_in[i+1..].iter() {
					let mut p = param.clone();
					p.make_ascii_lowercase();
					if p.chars().next().unwrap() != '-' { params.push(p.clone()) }
					else { break; }
				}
				let mut a = arg.clone();
				a.make_ascii_lowercase();
				res.insert(a.clone(), params);
			}
		}
		return Ok(res);
	}
}
