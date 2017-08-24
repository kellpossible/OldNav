//! A module with methods for `Airport` and other associated functions.

use navdata::waypoint::Waypoint;
use navdata::waypoint::WaypointInterface;
use navdata::coord::SphericalCoordinate;
use navdata::country::Country;
use std::fmt;
use std::rc::Rc;

/// An airport on earth.
pub struct Airport {
    /// This airport's waypoint.
    pub waypoint: Waypoint,
}

impl Airport {
    /// Constructor for `Airport`.
    pub fn new<S: Into<String>>(code: S, name: S, pos: SphericalCoordinate) -> Airport {
        return Airport { waypoint: Waypoint::new(code, name, pos, None) };
    }
}


impl WaypointInterface for Airport {
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

impl fmt::Debug for Airport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "Airport: {{code: {}, name: {}, pos: [{}, {}]}}",
            self.waypoint.code,
            self.waypoint.name,
            self.pos().lat(),
            self.pos().lon()
        );

    }
}
