use navdata::coord::SphericalCoordinate;
use std::collections::HashMap;
use navdata::country::Country;
use std::io::{Error, Result, BufReader, BufRead};
use std::fs::File;

pub struct Waypoint {
	pub code: String,
	pub name: String,
	pub pos: SphericalCoordinate,
	pub country: Option<Country>
}

pub trait WaypointInterface {
	fn code(&self) -> &str;
	fn name(&self) -> &str;
	fn pos(&self) -> &SphericalCoordinate;
}

impl Waypoint {
	pub fn new<S: Into<String>>(code: S, name: S, pos: SphericalCoordinate, country: Option<Country>) -> Waypoint {
		return Waypoint {
			code: code.into(),
			name: name.into(),
			pos: pos,
			country: country
		};
	}
}

impl WaypointInterface for Waypoint {
	fn code(&self) -> &str {
        return &self.code;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn pos(&self) -> &SphericalCoordinate {
        return &self.pos;
    }
}

/// Read Waypoints.txt file from x-plane's gns430 nav data
/// cm is a HashMap which maps countries to their icao identifier, and is used
/// during the reading of the waypoints.
pub fn read_waypoints(file_path: &str, cm: HashMap<String, Country>) -> Result<Vec<Waypoint>> {
	let mut waypoints = Vec::new();


    let f: File = match File::open(file_path) {
    	Ok(v) => v,
    	Err(e) => return Err(Error::new(e.kind(), 
    		format!("Unable to open waypoints file: {}", file_path)))
    };

    let bf = BufReader::new(&f);

    for line in bf.lines() {
    	let l = try!(line);
    	let split: Vec<&str> = l.split(",").collect();
    	let waypoint_code = split[0].to_string();
    	let lat: f64 = split[1].to_string().parse().unwrap();
    	let lon: f64 = split[2].to_string().parse().unwrap();

    	println!("{:?}", split);

    	waypoints.push(Waypoint::new(
    		waypoint_code, waypoint_code.clone(),
    		pos, None))
    }

    return Ok(wm);
}
