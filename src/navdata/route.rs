//! Routes and Legs

use navdata::waypoint::Waypoint;
use std::rc::Rc;
use std::fmt;
use linked_list::LinkedList;

/// Defines a route
///
/// # Examples
/// ```
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// # use oldnav_lib::navdata::waypoint::Waypoint;
/// # use oldnav_lib::navdata::route::{Leg, Route};
/// # use std::rc::Rc;
/// let p1 = SphericalCoordinate::from_geographic(0.0, 38.0, 144.0);
/// let c1 = Rc::new(Waypoint::new("1A", "Waypoint 1A", p1, None));
/// let p2 = SphericalCoordinate::from_geographic(0.0, 39.0, 144.0);
/// let c2 = Rc::new(Waypoint::new("1B", "Waypoint 1B", p2, None));
/// let p3 = SphericalCoordinate::from_geographic(0.0, 39.0, 145.0);
/// let c3 = Rc::new(Waypoint::new("1C", "Waypoint 1C", p3, None));
///
/// let leg1 = Leg::new(c1.clone(), c2.clone());
/// let leg2 = Leg::new(c2.clone(), c3.clone());
///
/// assert_eq!(leg1.start.name, c1.name);
///
/// let mut route = Route::new("a new route");
/// route.append_leg(leg1);
/// route.append_leg(leg2);
///
/// assert_eq!(route.first().unwrap().name, c1.name);
/// assert_eq!(route.last().unwrap().name, c3.name);
/// ```
#[derive(Debug)]
pub struct Route {
    /// Name of the `Route`
    pub name: String,

    /// Legs of the `Route`
    pub legs: LinkedList<Leg>,
}

//TODO TODO TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! ################################
// maybe a route should be comprised just of waypoints?
// what would be the deal with having SIDs and STARs?
// more research needs to be done here.


impl Route {
    /// Constructor for `Route`
    pub fn new<S: Into<String>>(name: S) -> Route {
        Route {
            name: name.into(),
            legs: LinkedList::new(),
        }
    }

    /// Move all legs from `other` route to the end of this route.
    /// This reuses all the nodes from `other` and moves them into
    /// `self`. After this operation `other` route is emptied.
    /// This operation should be in O(1) time and memory.
    pub fn append(&mut self, other: &mut Route) {
        self.legs.append(&mut other.legs);
    }

    /// Apend a leg to the end of this route
    pub fn append_leg(&mut self, leg: Leg) {
        self.legs.push_back(leg);
    }

    /// Returns the number of waypoints in the route.
    pub fn len(&self) -> usize {
        return self.legs.len();
    }

    /// Returns the number of legs in the route.
    pub fn len_legs(&self) -> usize {
        return self.legs.len();
    }

    /// Insert a leg into this route at the given index position.
    pub fn insert_leg(&mut self, index: usize, leg: Leg) {
        self.legs.insert(index, leg);
    }

    /// Get the first leg in the route.
    pub fn first(&self) -> Option<&Leg> {
        return self.legs.back();
    }

    /// Get the last leg in the route.
    pub fn last(&self) -> Option<&Leg> {
        return self.legs.front();
    }
}


/// Defines a leg in a `Route`
pub struct Leg {
    /// Start waypoint of the `Leg`
    pub start: Rc<Waypoint>,

    /// End waypoint of the `Leg`
    pub end: Option<Rc<Waypoint>>,
}

impl Leg {
    /// Constructor for `Leg`
    pub fn new(start: Rc<Waypoint>, end: Rc<Waypoint>) -> Leg {
        Leg {
            start: start,
            end: Some(end),
        }
    }

    pub fn new_single(start: Rc<Waypoint>) -> Leg {
        Leg {
            start: start,
            end: None,
        }
    }
}

impl fmt::Debug for Leg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,
                      "Leg: {{Start<{:p}>: {:?}, End<{:p}>: {:?} }}",
                      &*self.start,
                      self.start,
                      &*self.end,
                      self.end);

    }
}
