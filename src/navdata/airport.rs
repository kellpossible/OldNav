use navdata::waypoint::Waypoint;
use navdata::waypoint::WaypointInterface;
use navdata::coord::SphericalCoordinate;
use navdata::country::Country;

pub struct Airport {
    pub waypoint: Waypoint,
}

impl Airport {
    pub fn new<S: Into<String>>(code: S, name: S, pos: SphericalCoordinate, country: Option<Country>) -> Airport {
        return Airport { waypoint: Waypoint::new(code, name, pos, country) };
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
