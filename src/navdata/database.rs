//! The Navdata Database - Loaded from the x-plane GNS430 database.

use navdata::waypoint::Waypoint;
// use navdata::multihash::MultiHash;
// use navdata::waypoint::WaypointInterface;
use navdata::country::Country;
use navdata::coord::SphericalCoordinate;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;
use std::fmt;
use chrono::{DateTime, UTC, TimeZone};
use chrono::format::ParseResult;

/// A navigation database
pub struct Database {
    /// Where all the fixes are stored in the database
    pub fixes: Vec<Rc<Waypoint>>,
    // note: if I want to make waypoint mutable, or country mutable,
    // I may need to put them in a Rc<RefCell<Waypoint>>.

    // pub airports: Vec<Airport<'a>>,
    // pub waypoints: MultiHash<String, &'a WaypointInterface>,

    /// Where all the countries are stored in the database
    pub countries: HashMap<String, Rc<Country>>,

    /// Information about the current AIRAC cycle loaded into this navigation
    /// database.
    pub cycle_info: CycleInfo
}

impl Database {
    /// Constructor for `Database`
    pub fn new(navdata_dir: PathBuf, resources_dir: PathBuf) -> Database {
        let countries_path = resources_dir.join("icao_countries.txt");
        let countries_path = countries_path.to_str().unwrap();

        let waypoints_path = navdata_dir.join("waypoints.txt");
        let waypoints_path = waypoints_path.to_str().unwrap();

        let cycle_info_path = navdata_dir.join("cycle_info.txt");
        let cycle_info_path = cycle_info_path.to_str().unwrap();

        let mut db = Database {
            countries: HashMap::new(),
            fixes: Vec::new(),
            cycle_info: read_cycle_info(cycle_info_path)
        };

        db.read_countries(countries_path);
        db.read_fixes(waypoints_path);

        return db;
    }

    /// Read Waypoints.txt file from x-plane's gns430 nav data to obtain fixes.
    fn read_fixes(&mut self, file_path: &str) {
        let f = File::open(file_path).expect(&format!("Cannot open file {}", file_path));

        let bf = BufReader::new(&f);

        for line in bf.lines() {
            let l = line.unwrap();
            let split: Vec<&str> = l.split(",").collect();
            let waypoint_code = split[0].to_string();
            let lat: f64 = split[1].to_string().parse().unwrap();
            let lon: f64 = split[2].to_string().parse().unwrap();
            let pos = SphericalCoordinate::from_geographic(0.0, lat, lon);


            // unwrap the option and get a counted reference to the country
            let country = match self.countries.get(split[3]) {
                None => None,
                Some(v) => {Some(v.clone())}
            };

            self.fixes.push(Rc::new(Waypoint::new(waypoint_code.clone(),
                                                  waypoint_code,
                                                  pos,
                                                  country)));
        }

    }

    /// Read countries in from a txt file.
    ///
    /// Basically it just maps ICAO codes to the country names,
    /// see icao_countries.txt for an example file format.
    fn read_countries(&mut self, file_path: &str) {

        let f = File::open(file_path).expect(&format!("Cannot open file {}", file_path));

        let bf = BufReader::new(&f);

        for line in bf.lines() {
            let l = line.unwrap();
            let split: Vec<&str> = l.split("\t").collect();
            let country_code = split[0].to_string();
            let country_name = split[1].to_string();

            self.countries.insert(country_code.clone(),
                                  Rc::new(Country::new(country_code, country_name)));
        }

    }
}


impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
                      "Database: {{n_fixes: {}, n_airports: {}, n_countries: {}}}",
                      self.fixes.len(),
                      0,
                      self.countries.len()
                      );

    }
}

/// AIRAC Cycle Info
///
/// # Examples
/// ```rust,no_run
/// # use std::path::PathBuf;
/// # use oldnav_lib::navdata::database::{CycleInfo, Database};
/// let db = Database::new(PathBuf::new(), PathBuf::new());
/// let cycle_info = &db.cycle_info;
///
/// println!("airac cycle number: {}", cycle_info.airac_cycle);
/// ```
#[derive(Debug)]
pub struct CycleInfo {
    /// The cycle number
    pub airac_cycle: i32,
    /// Version of the cycle format?
    pub version: i32,
    /// The date this cycle comes into effect (and becomes valid)
    pub valid_from: DateTime<UTC>,
    /// The date after which this cycle expires and becomes invalid
    pub valid_to: DateTime<UTC>,
    /// Message embedded with the cycle data
    pub message: String
}

impl CycleInfo {
    /// Constructor for `CycleInfo`
    pub fn new(airac_cycle: i32,
    version: i32,
    valid_from: DateTime<UTC>,
    valid_to: DateTime<UTC>,
    message: String) -> CycleInfo {
        CycleInfo {
            airac_cycle: airac_cycle,
    version: version,
    valid_from: valid_from,
    valid_to: valid_to,
    message: message
        }
    }
}


/// Read cycle info from GNS430 nav database
fn read_cycle_info(file_path: &str) -> CycleInfo {
    let f = File::open(file_path).expect(&format!("Cannot open file {}", file_path));
    let bf = BufReader::new(&f);
    let mut values: HashMap<String, String> = HashMap::new();

    let mut message: String = String::new();

    for line in bf.lines() {
        let line_str = line.unwrap();
        let split: Vec<&str> = line_str.split(":").collect();

        if split.len() > 0 {
            let lhs = split[0].trim();

            if lhs == "AIRAC cycle"  {
                let rhs = split[1].trim();
                values.insert(String::from("airac_cycle"), String::from(rhs));
            } else if lhs == "Version" {
                let rhs = split[1].trim();
                values.insert(String::from("version"), String::from(rhs));
            } else if lhs == "Valid (from/to)" {
                let rhs = split[1];

                let split_dates: Vec<&str> = rhs.split("-").collect();
                let from_date = split_dates[0].trim();
                let to_date = split_dates[1].trim();

                values.insert(String::from("from_date"), String::from(from_date));
                values.insert(String::from("to_date"), String::from(to_date));
            } else if split.len() == 1 {
                message.push_str(split[0]);
                message.push(' ');
            }
        }
    }

    let airac_cycle = values.get("airac_cycle").unwrap().parse::<i32>().unwrap();
    let version = values.get("version").unwrap().parse::<i32>().unwrap();

    let from_date_str = values.get("from_date").unwrap();
    let from_date = parse_date_str(from_date_str).unwrap();

    let to_date_str = values.get("from_date").unwrap();
    let to_date = parse_date_str(to_date_str).unwrap();

    return CycleInfo::new(airac_cycle,version,from_date, to_date, message);
}


/// parse a string with a format like 10/JUN/2016 into a `DateTime<UTC>` object.
fn parse_date_str(date_str: &str) -> ParseResult<DateTime<UTC>> {
    let mut datetime_str = String::from(date_str.clone());
    datetime_str.push_str(" 00:00:00");
    return UTC.datetime_from_str(&datetime_str, "%d/%b/%Y %H:%M:%S");
}