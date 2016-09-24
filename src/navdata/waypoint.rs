use navdata::coord::SphericalCoordinate;
use std::collections::HashMap;
use navdata::country::Country;
use std::io::{Error, Result, BufReader, BufRead};
use std::fs::File;
use std::fmt;

pub struct Waypoint<'a> {
    pub code: String,
    pub name: String,
    pub pos: SphericalCoordinate,
    pub country: Option<&'a Country>,
}

pub trait WaypointInterface {
    fn code(&self) -> &str;
    fn name(&self) -> &str;
    fn pos(&self) -> &SphericalCoordinate;
}

impl<'a> Waypoint<'a> {
    pub fn new<S: Into<String>>(code: S,
                                name: S,
                                pos: SphericalCoordinate,
                                country: Option<&'a Country>)
                                -> Waypoint<'a> {
        return Waypoint {
            code: code.into(),
            name: name.into(),
            pos: pos,
            country: country,
        };
    }
}

impl<'a> WaypointInterface for Waypoint<'a> {
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

impl<'a> fmt::Debug for Waypoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let country_str = match self.country {
            None => "None",
            Some(c) => &*c.name,
        };
        return write!(f,
                      "Waypoint {{code: {}, name: {}, pos: [{},{}], country: {}}}",
                      self.code,
                      self.name,
                      self.pos.lat(),
                      self.pos.lon(),
                      country_str);

    }
}

/// Read Waypoints.txt file from x-plane's gns430 nav data
/// cm is a HashMap which maps countries to their icao identifier, and is used
/// during the reading of the waypoints.
pub fn read_waypoints<'a>(file_path: &str,
                          cm: &'a HashMap<String, Country>)
                          -> Result<Vec<Waypoint<'a>>> {
    let mut waypoints = Vec::new();


    let f: File = match File::open(file_path) {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(e.kind(),
                                  format!("Unable to open waypoints file: {}", file_path)))
        }
    };

    let bf = BufReader::new(&f);

    for line in bf.lines() {
        let l = try!(line);
        let split: Vec<&str> = l.split(",").collect();
        let waypoint_code = split[0].to_string();
        let lat: f64 = split[1].to_string().parse().unwrap();
        let lon: f64 = split[2].to_string().parse().unwrap();
        let pos = SphericalCoordinate::from_geographic(0.0, lat, lon);


        let country: Option<&'a Country> = cm.get(split[3]);

        // println!("{:?}", split);

        waypoints.push(Waypoint::new(waypoint_code.clone(), waypoint_code, pos, country))
    }

    return Ok(waypoints);
}
