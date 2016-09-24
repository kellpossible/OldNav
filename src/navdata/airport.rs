//! A module with methods for `Airport` and other associated functions.

use navdata::waypoint::Waypoint;
use navdata::waypoint::WaypointInterface;
use navdata::coord::SphericalCoordinate;
use navdata::country::Country;
use std::fmt;

/// An airport on earth.
pub struct Airport<'a> {
    /// This airport's waypoint.
    pub waypoint: Waypoint<'a>,
}

impl<'a> Airport<'a> {
    /// Constructor for `Airport`.
    pub fn new<S: Into<String>>(code: S,
                                name: S,
                                pos: SphericalCoordinate,
                                country: Option<&'a Country>)
                                -> Airport<'a> {
        return Airport { waypoint: Waypoint::new(code, name, pos, country) };
    }
}


impl<'a> WaypointInterface for Airport<'a> {
    fn code(&self) -> &str {
        return &self.waypoint.code;
    }

    fn name(&self) -> &str {
        return &self.waypoint.name;
    }

    fn pos(&self) -> &SphericalCoordinate {
        return &self.waypoint.pos;
    }
}

impl<'a> fmt::Debug for Airport<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
                      "Airport: {{code: {}, name: {}, pos: [{}, {}]}}",
                      self.waypoint.code,
                      self.waypoint.name,
                      self.pos().lat(),
                      self.pos().lon());

    }
}
