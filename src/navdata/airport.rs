use navdata::waypoint::Waypoint;
use navdata::waypoint::WaypointInterface;
use navdata::coord::SphericalCoordinate;
use navdata::country::Country;

pub struct Airport<'a> {
    pub waypoint: Waypoint<'a>,
}

impl<'a> Airport<'a> {
    pub fn new<S: Into<String>>(code: S, name: S, pos: SphericalCoordinate, country: Option<&'a Country>) -> Airport<'a> {
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
