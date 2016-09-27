//! The Navdata Database - Loaded from the x-plane GNS430 database.

use navdata::waypoint::Waypoint;
// use navdata::multihash::MultiHash;
// use navdata::waypoint::WaypointInterface;
use navdata::country::Country;
// use navdata::airport::Airport;
use navdata::coord::SphericalCoordinate;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;
use std::fmt;

/// A navigation database
pub struct Database {
    /// Where all the fixes are stored in the database
    pub fixes: Vec<Rc<Waypoint>>,
    // pub airports: Vec<Airport<'a>>,
    // pub waypoints: MultiHash<String, &'a WaypointInterface>,

    /// Where all the countries are stored in the database
    pub countries: HashMap<String, Rc<Country>>,
}

impl Database {
    /// Constructor for `Database`
    pub fn new(navdata_dir: PathBuf, resources_dir: PathBuf) -> Database {
        let countries_path = resources_dir.join("icao_countries.txt");
        let countries_path = countries_path.to_str().unwrap();

        let waypoints_path = navdata_dir.join("waypoints.txt");
        let waypoints_path = waypoints_path.to_str().unwrap();

        let mut db = Database {
            countries: HashMap::new(),
            fixes: Vec::new(),
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
