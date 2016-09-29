//! Routes and Legs

use navdata::waypoint::Waypoint;
use std::rc::Rc;
use linked_list::LinkedList;

/// Defines a route
///
/// # Examples
/// ```
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// # use oldnav_lib::navdata::waypoint::Waypoint;
/// # use oldnav_lib::navdata::route::Route;
/// # use std::rc::Rc;
/// let p1 = SphericalCoordinate::from_geographic(0.0, 38.0, 144.0);
/// let c1 = Rc::new(Waypoint::new("1A", "Waypoint 1A", p1, None));
/// let p2 = SphericalCoordinate::from_geographic(0.0, 39.0, 144.0);
/// let c2 = Rc::new(Waypoint::new("1B", "Waypoint 1B", p2, None));
/// let p3 = SphericalCoordinate::from_geographic(0.0, 39.0, 145.0);
/// let c3 = Rc::new(Waypoint::new("1C", "Waypoint 1C", p3, None));
///
/// let mut route = Route::new("a new route");
/// route.append_waypoint(c1.clone());
/// route.append_waypoint(c2.clone());
/// route.append_waypoint(c3.clone());
///
/// assert_eq!(route.first().unwrap().name, c1.name);
/// assert_eq!(route.last().unwrap().name, c3.name);
/// ```
#[derive(Debug)]
pub struct Route {
    /// Name of the `Route`
    pub name: Option<String>,

    /// Legs of the `Route`
    pub waypoints: LinkedList<Rc<Waypoint>>,
}

// TODO TODO TODO!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! ################################
// maybe a route should be comprised just of waypoints?
// what would be the deal with having SIDs and STARs?
// more research needs to be done here.


impl Route {
    /// Constructor for `Route`
    pub fn new(name: Option<String>) -> Route {
        Route {
            name: name,
            waypoints: LinkedList::new(),
        }
    }

    /// Move all legs from `other` route to the end of this route.
    /// This reuses all the nodes from `other` and moves them into
    /// `self`. After this operation `other` route is emptied.
    /// This operation should be in O(1) time and memory.
    pub fn append(&mut self, other: &mut Route) {
        self.waypoints.append(&mut other.waypoints);
    }

    /// Insert a waypoint into this route at the given index position.
    pub fn insert_waypoint(&mut self, index: usize, waypoint: Rc<Waypoint>) {
        self.waypoints.insert(index, waypoint);
    }

    /// Append a waypoint to the end of this route.
    pub fn append_waypoint(&mut self, waypoint: Rc<Waypoint>) {
        self.waypoints.push_back(waypoint);
    }

    /// Returns the number of waypoints in the route.
    pub fn len(&self) -> usize {
        return self.waypoints.len();
    }

    /// Get the first waypoint in the route.
    pub fn first(&self) -> Option<&Rc<Waypoint>> {
        return self.waypoints.front();
    }

    /// Get the last waypoint in the route.
    pub fn last(&self) -> Option<&Rc<Waypoint>> {
        return self.waypoints.back();
    }
}
