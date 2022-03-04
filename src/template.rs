


pub mod template {

	use crate::project::*;
	//use crate::project::{Language};
	use std::fs::File;
	use std::path::Path;
	use serde::{Deserialize, Serialize};
	use std::collections::HashMap;


	pub struct Template {
		pub lang               : project::Language,
		pub source             : String,
		pub buildtool          : String,
		pub buildtool_template : String,
		pub has_extra          : bool,
		pub extra_template     : String
	}

	#[derive(Debug, Deserialize, Serialize, Clone)]
	pub struct JSON_template {
		pub name               : String,
		pub source             : String,
		pub buildtool          : String,
		pub buildtool_template : String,
		pub has_extra          : bool,
		pub extra_template     : String
	}

	impl Template {

		pub fn from(lang: project::Language, jt: JSON_template) -> Template {
			return Template {
				lang               : lang,
				source             : jt.source,
				buildtool          : jt.buildtool,
				buildtool_template : jt.buildtool_template,
				has_extra          : jt.has_extra,
				extra_template     : jt.extra_template
			};
		}
	}

	pub fn get_templates(lang: project::Language) -> Result<HashMap<String, Template>, &'static str> {
		let template_path = Path::new(".proinit/templates.json");
		let file = match File::open(template_path) {
			Ok(file) => file,
			Err(err) => { return Err("could not open file"); }
		};
		let json_templates: Vec<JSON_template> = match serde_json::from_reader(file) {
			Ok(vector) => vector,
			Err(err) => { return Err("could not parse file"); }
		};

		let mut res = HashMap::new();

		for template in json_templates.iter() {
			res.insert(template.name.clone(), Template::from(lang, template.clone()));
		}
		return Ok(res);
	}


}
