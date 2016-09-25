extern crate oldnav_lib;
use oldnav_lib::navdata::airport::Airport;
use oldnav_lib::navdata::coord::SphericalCoordinate;
use oldnav_lib::navdata::waypoint::WaypointInterface;
use oldnav_lib::navdata::waypoint;
use oldnav_lib::navdata::country;
use oldnav_lib::navdata::country::Country;
use std::collections::HashMap;
use std::env::current_exe;

fn main() {
    
    let mut exe_dir = current_exe().unwrap();
    exe_dir.pop();

    let resources_dir = exe_dir.clone().join("resources");


    let mut airport = Airport::new("YMML",
                                   "Melbourne International",
                                   SphericalCoordinate::from_geographic(0.0, 38.0, 148.0),
                                   None);

    let countries_path = resources_dir.join("icao_countries.txt");
    let countries_path = countries_path.to_str().unwrap();
    let cm: HashMap<String, Country> = country::read_countries(countries_path)
        .unwrap();

    let waypoints_path = resources_dir.join("Waypoints.txt");
    let waypoints_path = waypoints_path.to_str().unwrap();
    let waypoints = waypoint::read_waypoints(waypoints_path, &cm).unwrap();


    let pos = SphericalCoordinate::new(10.0, 3.0*std::f64::consts::PI, 0.0);
    println!("{:?}", pos);


    airport.waypoint.pos = SphericalCoordinate::from_geographic(10.0, 38.0, 150.0);
}
