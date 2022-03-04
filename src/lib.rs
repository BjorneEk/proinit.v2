

pub mod interface;
pub mod args;
pub mod project;
pub mod template;
//use std::fs;


pub mod config {
	use std::env;

	pub fn config_dir() -> Result<String, &'static str> {
		match home::home_dir() {
			Some(path) => match path.to_str().clone() {
				Some(str) => Ok(String::from(str)),
				None      => Err("could not find home directory")
			},
			None => Err("could not find home directory")
		}
	}

	pub fn current_dir() -> Result<String, &'static str> {
		match env::current_dir() {
			Ok(path) => match path.to_str().clone() {
				Some(str) => Ok(String::from(str)),
				None      => Err("could not get current directory")
			},
			Err(_e) => Err("could not get current directory")
		}
	}
}
