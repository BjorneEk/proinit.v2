use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use proinit::config::*;
use proinit::args::*;
use proinit::interface::*;

/*--------------------------------------------------------------*
 *                  proinit rewritten in rust
 *    @author Gustaf Franzén :: https://github.com/BjorneEk;
 *
 *   original proinit repo: https://github.com/BjorneEk/proinit
 *
 *-------------------------------------------------------------*/


fn main() -> std::io::Result<()>  {

	let name = args::project_name();
	println!("name: {}",name);
	if name.chars().next().unwrap() == '-' { interface::exit_error(String::from("project name not specified"), true); }
	let args = match args::get_args() {
		Ok(args) => args,
		Err(err) => { interface::exit_error(String::from(err), false);  HashMap::new() }
	};

	for (key, val) in args.iter() {
		print!("{}", key);
		for p in val {
			print!(" {}", p);
		}
		println!("");
	}
	Ok(())

}
