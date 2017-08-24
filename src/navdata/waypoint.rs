//! A module with methods for `Waypoint` and other associated functions and interfaces.

use navdata::coord::SphericalCoordinate;
use navdata::airport::Airport;
use std::fmt;
use std::rc::Rc;

/// An ICAO waypoint
///
/// # Examples
///
/// ```
/// # use oldnav_lib::navdata::waypoint::Waypoint;
/// # use oldnav_lib::navdata::country::Country;
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// # use std::rc::Rc;
/// let country = Country::new("AG", "Solomon Islands");
/// let pos = SphericalCoordinate::from_geographic(0.0, -9.66483, 161.02166);
/// let waypoint = Waypoint::new("ERVOS", "ERVOS", pos, Some(Rc::new(country)));
/// ```
pub struct Waypoint {
    /// ICAO airport code
    pub code: String,

    /// Name of airport
    pub name: String,

    /// Position of airport
    pub pos: SphericalCoordinate,

    /// `Airport` terminal area containing this `Waypoint`
    pub airport: Option<Rc<Airport>>,
}

/// A common interface for accessing objects which can provide waypoint information.
pub trait WaypointInterface {
    /// Get the ICAO code for this waypoint.
    fn code(&self) -> &str;

    /// Get the name of this waypoint.
    fn name(&self) -> &str;

    /// Get the position of this waypoint.
    fn pos(&self) -> &SphericalCoordinate;
}

impl Waypoint {
    /// Constructor for `Waypoint`.
    pub fn new<S: Into<String>>(
        code: S,
        name: S,
        pos: SphericalCoordinate,
        airport: Option<Rc<Airport>>,
    ) -> Waypoint {
        return Waypoint {
            code: code.into(),
            name: name.into(),
            pos: pos,
            airport: airport,
        };
    }
}

impl WaypointInterface for Waypoint {
    fn code(&self) -> &str {
        return &self.code;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn pos(&self) -> &SphericalCoordinate {
        return &self.pos;
    }
}

impl WaypointInterface for UnlinkedWaypoint {
    fn code(&self) -> &str {
        return &self.code;
    }

    fn name(&self) -> &str {
        return &self.name;
    }

    fn pos(&self) -> &SphericalCoordinate {
        return &self.pos;
    }
}

#[derive(Debug)]
pub struct UnlinkedWaypoint {
    /// ICAO airport code
    pub code: String,

    /// Name of airport
    pub name: String,

    /// Position of airport
    pub pos: SphericalCoordinate,

    /// `Airport` terminal area containing this `Waypoint`
    pub airport: Option<String>,
}


impl UnlinkedWaypoint {
    /// Constructor for `Waypoint`.
    pub fn new<S: Into<String>>(
        code: S,
        name: S,
        pos: SphericalCoordinate,
        airport: Option<String>,
    ) -> UnlinkedWaypoint {
        return UnlinkedWaypoint {
            code: code.into(),
            name: name.into(),
            pos: pos,
            airport: airport,
        };
    }
}


impl fmt::Debug for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let airport_str = match self.airport.as_ref() {
            None => "None",
            Some(airport) => airport.code(),
        };
        return write!(
            f,
            "Waypoint {{code: {}, name: {}, pos: [{},{}], country: {}}}",
            self.code,
            self.name,
            self.pos.lat(),
            self.pos.lon(),
            airport_str
        );

    }
}
