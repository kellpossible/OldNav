//! The Navdata Database - Loaded from the x-plane GNS430 database.

use navdata::waypoint::Waypoint;
use navdata::multihash::MultiHash;
// use navdata::waypoint::WaypointInterface;
use navdata::country::Country;
use navdata::coord::SphericalCoordinate;
use navdata::route::Route;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;
use std::fmt;
use chrono::{DateTime, UTC, TimeZone};
use chrono::format::ParseResult;
use std::mem;

/// The result of a query for waypoint or waypoints in `Database`
pub enum WaypointQueryResult<T> {
    /// `T` was found
    Found(T),

    /// `T` was found but it was too far.
    TooFar(T),

    /// `T` was not found at all.
    NotFound
}

impl<T> WaypointQueryResult<T> {
    /// Returns true if the result is `Found`.
    #[inline]
    pub fn is_found(self) -> bool {
        match self {
            WaypointQueryResult::Found(_) => true,
            WaypointQueryResult::TooFar(_) => false,
            WaypointQueryResult::NotFound => false,
        }
    }

    /// Returns true if the result is `None`.
    #[inline]
    pub fn is_none(self) -> bool {
        match self {
            WaypointQueryResult::Found(_) => false,
            WaypointQueryResult::TooFar(_) => false,
            WaypointQueryResult::NotFound => true,
        }
    }

    /// Returns true if the result is `TooFar`.
    #[inline]
    pub fn is_too_far(self) -> bool {
        match self {
            WaypointQueryResult::Found(_) => false,
            WaypointQueryResult::TooFar(_) => false,
            WaypointQueryResult::NotFound => true,
        }
    }

    /// Converts from `WaypointQueryResult<T> to `Option<T>`
    /// Returns a None if the result is not Found.
    #[inline]
    pub fn found(self) -> Option<T> {
        match self {
            WaypointQueryResult::Found(x) => Some(x),
            WaypointQueryResult::TooFar(_) => None,
            WaypointQueryResult::NotFound => None,
        }
    }

    /// Converts from `WaypointQueryResult<T> to `Option<T>`.
    /// Returns a None if the result is not TooFar.
    #[inline]
    pub fn too_far(self) -> Option<T> {
        match self {
            WaypointQueryResult::Found(_) => None,
            WaypointQueryResult::TooFar(x) => Some(x),
            WaypointQueryResult::NotFound => None,
        }
    }
}


/// A navigation database
pub struct Database {
    /// Where all the fixes are stored in the database
    pub fixes: Vec<Rc<Waypoint>>,

    /// hash of waypoints associated with their names
    pub waypoint_hash: MultiHash<String, Rc<Waypoint>>,

    // note: if I want to make waypoint mutable, or country mutable,
    // I may need to put them in a Rc<RefCell<Waypoint>>.
    //
    // pub airports: Vec<Airport<'a>>,
    // pub waypoints: MultiHash<String, &'a WaypointInterface>,
    /// Where all the countries are stored in the database
    pub countries: HashMap<String, Rc<Country>>,

    /// Where all the airways are stored in the database
    pub airways: HashMap<String, Rc<Route>>,

    /// Information about the current AIRAC cycle loaded into this navigation
    /// database.
    pub cycle_info: CycleInfo,
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

        let airways_path = navdata_dir.join("ats.txt");
        let airways_path = airways_path.to_str().unwrap();

        let mut db = Database {
            countries: HashMap::new(),
            airways: HashMap::new(),
            fixes: Vec::new(),
            waypoint_hash: MultiHash::new(),
            cycle_info: read_cycle_info(cycle_info_path),
        };

        db.read_countries(countries_path);
        db.read_fixes(waypoints_path);
        db.read_airways(airways_path);

        return db;
    }

    /// Read Waypoints.txt file from x-plane's gns430 nav data to obtain fixes.
    /// Needs to be called after read_countries in order to reference the countries.
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
                Some(v) => Some(v.clone()),
            };

            let waypoint =
                Waypoint::new(waypoint_code.clone(), waypoint_code.clone(), pos, country);
            self.insert_fix(waypoint)
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

    /// Read the airways into the database.
    /// Needs to be called after read_fixes()
    fn read_airways(&mut self, file_path: &str) {
        let f = File::open(file_path).expect(&format!("Cannot open file {}", file_path));

        let bf = BufReader::new(&f);

        let mut new_airway = Route::new(None);
        let mut first_leg: bool = true; //first leg in airway
        let mut valid_airway = true; // whether or not all the waypoints are valid

        for line in bf.lines() {
            let l = line.unwrap();

            if l == "" {
                if valid_airway {
                    // allow us to insert this airway into airways,
                    // and continue to create new airways without the new_airway
                    // value getting moved.
                    let a = Rc::new(mem::replace(&mut new_airway, Route::new(None)));
                    let s = a.name.clone();

                    self.airways.insert(s.unwrap(), a);
                }

            } else {
                let split: Vec<&str> = l.split(",").collect();
                let value_type = split[0].to_string();

                if value_type == "A" {
                    new_airway.name = Some(split[1].to_string());
                    valid_airway = true;
                    first_leg = true;
                }

                if value_type == "S" {
                    // read first point in the leg, only for the first leg
                    if first_leg {
                        let w1_code = split[1].to_string();
                        let lat1: f64 = split[2].to_string().parse().unwrap();
                        let lon1: f64 = split[3].to_string().parse().unwrap();
                        let p1 = SphericalCoordinate::from_geographic(0.0, lat1, lon1);

                        let result = self.match_waypoint_dist(&w1_code, &p1, 0.1);
                        if result.is_some() {
                            new_airway.append_waypoint(result.unwrap().clone());
                        } else {
                            valid_airway = false;
                        }
                    }

                    // second point in the leg for every leg except the first one.
                    let w2_code = split[4].to_string();
                    let lat2: f64 = split[5].to_string().parse().unwrap();
                    let lon2: f64 = split[6].to_string().parse().unwrap();
                    let p2 = SphericalCoordinate::from_geographic(0.0, lat2, lon2);

                    let result = self.match_waypoint_dist(&w2_code, &p2, 0.1);
                    if result.is_some() {
                        new_airway.append_waypoint(result.unwrap().clone());
                    } else {
                        valid_airway = false;
                    }

                    first_leg = false;
                }
            }

        }
    }

    /// Insert a fix waypoint into this database.
    pub fn insert_fix(&mut self, waypoint: Waypoint) {
        let waypoint_ref = Rc::new(waypoint);
        self.waypoint_hash.insert(waypoint_ref.name.clone(), waypoint_ref.clone());
        self.fixes.push(waypoint_ref);
    }

    //TODO add an enumset for waypoint type.

    /// Find a waypoint which most closely matches the supplied parameters.
    /// + code: the icao code for the waypoint
    /// + position and max_dist: max distance of the waypoint from the given position
    pub fn match_waypoint_dist(&self, code: &str, position: &SphericalCoordinate, max_dist: f64) -> Option<&Rc<Waypoint>> {
        let matching_waypoints = self.waypoint_hash.get(&String::from(code));

        if matching_waypoints.is_none() {
            println!("Warning: this waypoint does not exist in the database: {} [{},{}]",
                     code,
                     position.lat(),
                     position.lon());
            return None;
        } else {
            let matching_waypoints: &Vec<Rc<Waypoint>> = matching_waypoints.unwrap();

            println!("matching waypoings to {}", code);
            for waypoint in &*matching_waypoints {
                println!("{:?}", waypoint);
            }

            return Some(&matching_waypoints[0]);
        }
    }
}


impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
                      "Database: {{n_fixes: {}, n_airports: {}, n_countries: {}, n_airways: {}}}",
                      self.fixes.len(),
                      0,
                      self.countries.len(),
                      self.airways.len());

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
    pub message: String,
}

impl CycleInfo {
    /// Constructor for `CycleInfo`
    pub fn new(airac_cycle: i32,
               version: i32,
               valid_from: DateTime<UTC>,
               valid_to: DateTime<UTC>,
               message: String)
               -> CycleInfo {
        CycleInfo {
            airac_cycle: airac_cycle,
            version: version,
            valid_from: valid_from,
            valid_to: valid_to,
            message: message,
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

            if lhs == "AIRAC cycle" {
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

    return CycleInfo::new(airac_cycle, version, from_date, to_date, message);
}


/// parse a string with a format like 10/JUN/2016 into a `DateTime<UTC>` object.
fn parse_date_str(date_str: &str) -> ParseResult<DateTime<UTC>> {
    let mut datetime_str = String::from(date_str.clone());
    datetime_str.push_str(" 00:00:00");
    return UTC.datetime_from_str(&datetime_str, "%d/%b/%Y %H:%M:%S");
}
