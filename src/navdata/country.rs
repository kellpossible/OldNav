use std::collections::HashMap;
use std::io::{Error, Result, BufReader, BufRead};
use std::fs::File;

pub struct Country {
    // the icao code
    pub code: String,
    pub name: String,
}

impl Country {
	pub fn new(code: String, name: String) -> Country {
		return Country {
			code: code,
			name: name
		};
	}
}


/// Read countries in from a txt file: see icao_countries.txt for an example file format
/// Basically it just maps ICAO codes to the country names
pub fn read_countries(file_path: &str) -> Result<HashMap<String, Country>> {
	let mut cm = HashMap::new();

    let f: File = match File::open(file_path) {
    	Ok(v) => v,
    	Err(e) => return Err(Error::new(e.kind(), 
    		format!("Unable to open countries map file: {}", file_path)))
    };

    let bf = BufReader::new(&f);

    for line in bf.lines() {
    	let l = try!(line);
    	let split: Vec<&str> = l.split("\t").collect();
    	let country_code = split[0].to_string();
    	let country_name = split[1].to_string();

    	cm.insert(
    		country_code.clone(), 
    		Country::new(country_code, country_name));
    }

    return Ok(cm);
}

