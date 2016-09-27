//! Routes and Legs

use navdata::waypoint::Waypoint;
use std::rc::Rc;

/// Defines a route
#[derive(Debug)]
pub struct Route {
	/// Name of the `Route`
	pub name: String,

	/// Legs of the `Route`
	pub legs: Vec<Leg>
}

/// Defines a leg in a `Route`
#[derive(Debug)]
pub struct Leg {
	/// Start waypoint of the `Leg`
	pub start: Rc<Waypoint>,

	/// End waypoint of the `Leg`
	pub end: Rc<Waypoint>
}

