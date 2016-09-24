extern crate oldnav_lib;
use oldnav_lib::navdata::airport::Airport;
use oldnav_lib::navdata::coord::SphericalCoordinate;
use oldnav_lib::navdata::waypoint::WaypointInterface;
use oldnav_lib::navdata::country;
use oldnav_lib::navdata::country::Country;
use std::collections::HashMap;


fn main() {
	#![warn(missing_docs)]
	let mut airport = Airport::new(
		"YMML",
		"Melbourne International", 
		SphericalCoordinate::from_geographic(0.0, 38.0, 148.0),
		None);

	let cm: HashMap<String, Country> = country::read_countries("resources/icao_countries.txt").unwrap();

    println!("{}", airport.name());
    println!("{}", airport.code());

    println!("{:?}", airport.pos());

    airport.waypoint.pos = SphericalCoordinate::from_geographic(10.0, 38.0, 150.0);

    println!("{}", airport.pos().fmt_geographic());
}
