//! A module with methods for `Waypoint` and other associated functions and interfaces.

use navdata::coord::SphericalCoordinate;
use std::collections::HashMap;
use navdata::country::Country;
use std::io::{Error, Result, BufReader, BufRead};
use std::fs::File;
use std::fmt;

/// An ICAO waypoint
///
/// # Examples
///
/// ```
/// # use oldnav_lib::navdata::waypoint::{Waypoint, read_waypoints};
/// # use oldnav_lib::navdata::country;
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// let country = country::Country::new("AG", "Solomon Islands");
/// let pos = SphericalCoordinate::from_geographic(0.0, -9.66483, 161.02166);
/// let waypoint = Waypoint::new("ERVOS", "ERVOS", pos, Some(&country));
/// ```
pub struct Waypoint<'a> {
    /// ICAO airport code
    pub code: String,

    /// Name of airport
    pub name: String,

    /// Position of airport
    pub pos: SphericalCoordinate,

    /// `Country` containing this `Waypoint`
    pub country: Option<&'a Country>,
}

/// A common interface for accessing objects which can provide waypoint information.
pub trait WaypointInterface {
    /// Get the ICAO code for this waypoint.
    fn code(&self) -> &str;

    /// Get the name of this waypoint.
    fn name(&self) -> &str;

    /// Get the position of this waypoint.
    fn pos(&self) -> &SphericalCoordinate;
}

impl<'a> Waypoint<'a> {
    /// Constructor for `Waypoint`.
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

/// Read Waypoints.txt file from x-plane's gns430 nav data.
///
/// cm is a `HashMap` which maps countries to their icao identifier, and is used
/// during the reading of the waypoints.
///
/// # Examples
///
/// ```rust,no_run
/// # use oldnav_lib::navdata::waypoint::{Waypoint, read_waypoints};
/// # use oldnav_lib::navdata::country;
/// # use std::collections::HashMap;
///
/// let countries_map = country::read_countries("some/path/to/icao_countries.txt").unwrap();
/// let mut waypoints = read_waypoints("some/path/to/Waypoints.txt", &countries_map).unwrap();
///
/// for waypoint in &waypoints {
///     println!("{}", waypoint.code);
/// }
///
/// let waypoint = &mut waypoints[0];
/// waypoint.pos.set_lat(30.0);
///
/// ```
///
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

        waypoints.push(Waypoint::new(waypoint_code.clone(), waypoint_code, pos, country))
    }

    return Ok(waypoints);
}
