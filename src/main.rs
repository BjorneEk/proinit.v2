use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use proinit::config::*;
use proinit::args::*;
use proinit::interface::*;

use proinit::project::*;
use proinit::template::*;

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
	//if name.chars().next().unwrap() == '-' { interface::exit_error(String::from("project name not specified"), true); }
	match name.as_str() {
		"-h"  | "--help"             => { interface::print_help();            std::process::exit(0) },
		"-lt" | "--list-templates"   => { println!("no templates currently"); std::process::exit(0) },
		"-ll" | "--list-languages"   => { interface::list_languages();        std::process::exit(0) },
		"-lb" | "--list-build-tools" => { interface::list_buildtools();       std::process::exit(0) },
		_ => {},
	}


	let args = match args::get_args() {
		Ok(args) => args,
		Err(err) => { interface::exit_error(String::from(err), false);  HashMap::new() }
	};

	let proj = project::Project::from(name, args.clone());

	let templates = match template::get_templates(project::Language::C) {
		Ok(templates) => templates,
		Err(err)      => { println!("error: {}", err); std::process::exit(0) }
	};

	for (key, val) in args.iter() {
		println!(" TEMPLATE: {} ", key);
	}


	println!("name: {}, language: {:?}, buildtool: {:?}, template {}", proj.name, proj.language.to_string(), proj.buildtool.to_string(), proj.template);
	for (key, val) in args.iter() { match key.as_str() {
		"-h"  | "--help"             => { interface::print_help(); std::process::exit(0) },
		"-lt" | "--list-templates"   => { println!("no templates currently") },
		"-ll" | "--list-languages"   => { interface::list_languages(); std::process::exit(0) },
		"-lb" | "--list-build-tools" => { interface::list_buildtools(); std::process::exit(0) },
		"-l"  | "--language" | "-bt" | "--build-tool" | "-t"  | "--template" => {}
		_ => interface::exit_error(String::from("unknown argument"), false)
	}}
	Ok(())

}
