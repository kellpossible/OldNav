//! Spherical and Geographic coordinates.
//!
//! TODO: more explaination here

use nalgebra::{Vector3, Dot, Vector2};
use std::f64::consts::PI;
use std::f64::*;
use std::fmt;
use navdata::geohash;

static TWO_PI: f64 = PI * 2.0;
static HALF_PI: f64 = PI / 2.0;

/// Mean sea level on earth
pub static EARTH_MSL_RADIUS: f64 = 6371008.8;

/// Represents a coordinate in the spherical coordinate system.
///
/// The normal range for the variables is as follows:
///
/// |name  |   range       |
/// |------|---------------|
/// | r    | 0 -> infinity |
/// |theta |  (0 -> 2PI)   |
/// |phi   |  (0 -> PI)  |
///
/// See: http://mathworld.wolfram.com/SphericalCoordinates.html for more details.
///
/// The design decision to combine Spherical with Geographical coordinates was made
/// beceause many of the algorithms available, and literature deal with the spherical
/// coordinate system, so it is easier to implement it with that. It is fairly low cost
/// to go between the two representations, and this only really occurs twice, once in, and
/// once out to display. Most of the heavy calculations will be while the coordinate is
/// treated as a spherical coordinate.
///
///
/// # Examples
///
/// It is very common to instantiate from a geographical point and then access those values later
/// in the form of latitude and longitude:
///
/// ```
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// # let accuracy = 0.0001;
///
/// let pos = SphericalCoordinate::from_geographic(324.567, 82.123, 23.452);
/// println!("{},{}", pos.lat(), pos.lon());
///
/// # assert!((pos.alt()-324.567).abs() < accuracy);
/// # assert!((pos.lat()-82.123).abs() < accuracy);
/// # assert!((pos.lon()-23.452).abs() < accuracy);
/// ```
///
/// If you are manually setting the r, theta or phi values you need to ensure that your values
/// fall within the range specified, or that you use the `rectify_bounds()` method to
/// transform the coordinate into the specified range after you set the value
///
/// ```
/// # use oldnav_lib::navdata::coord::SphericalCoordinate;
/// # use std::f64::consts::PI;
/// let accuracy = 0.0001;
///
/// let mut pos = SphericalCoordinate::new(30.0, 0.0, PI);
/// println!("{:?}", pos);
///
/// # assert!((pos.r-30.0).abs() < accuracy);
/// # assert!((pos.theta-0.0).abs() < accuracy);
/// # assert!((pos.phi-PI).abs() < accuracy);
///
/// // negative theta puts coordinate outside bounds of (0 -> 2PI)
/// pos.theta = -1.0 * PI;
/// pos.rectify_bounds_inplace();
/// println!("{:?}", pos);
///
/// // test to see that coordinate is now properly within the bounds
/// # assert!((pos.r-30.0).abs() < accuracy);
/// assert!((pos.theta-PI).abs() < accuracy);
/// assert!((pos.phi-PI).abs() < accuracy);
///
/// ```
///
#[derive(Copy, Clone)]
pub struct SphericalCoordinate {
    //todo: replace this struct with a trait for Vector3
    /// Radius component
    pub r: f64,

    /// Theta component
    pub theta: f64,

    /// Phi component
    pub phi: f64,
}


impl SphericalCoordinate {
    /// Constructor for `SphericalCoordinate`
    ///
    /// # Examples
    ///
    /// ```
    /// # use oldnav_lib::navdata::coord::SphericalCoordinate;
    /// # use std::f64::consts::PI;
    /// # let accuracy = 0.0001;
    ///
    /// let pos = SphericalCoordinate::new(10.0, PI, 0.0);
    /// println!("theta: {:?}", pos.theta);
    ///
    /// # assert!((pos.r-10.0).abs() < accuracy);
    /// # assert!((pos.theta-PI).abs() < accuracy);
    /// # assert!((pos.phi-0.0).abs() < accuracy);
    /// ```
    ///
    /// When the values exceed the normal bountaries for the spherical coordinate system, they
    /// are wrapped back around to the equivalent angle (rectified) to ensure they stay within
    /// the expected values for other calculations:
    ///
    /// ```
    /// # use oldnav_lib::navdata::coord::SphericalCoordinate;
    /// # use std::f64::consts::PI;
    ///
    /// let accuracy = 0.0001;
    /// let pos = SphericalCoordinate::new(10.0, 3.0*PI, 0.0);
    ///
    /// // test to see if values are assigned accurately
    /// assert!((pos.r-10.0).abs() < accuracy);
    /// // theta has been rectified to be within boundary (0 -> 2PI)
    /// assert!((pos.theta-PI).abs() < accuracy);
    /// assert!((pos.phi-0.0).abs() < accuracy);
    /// ```
    ///
    pub fn new(r: f64, theta: f64, phi: f64) -> SphericalCoordinate {
        let mut coord = SphericalCoordinate {
            r: r,
            theta: theta,
            phi: phi,
        };
        coord.rectify_bounds_inplace();
        return coord;
    }

    /// Create a new SphericalCoordinate from geographic coordinate.
    ///
    /// **alt**: metres above the surface as defined by `EARTH_MSL_RADIUS`
    ///
    /// **lat**: latitude in degrees
    ///
    /// **lon**: longitude in degrees
    ///
    /// # Examples
    ///
    ///
    pub fn from_geographic(alt: f64, lat: f64, lon: f64) -> SphericalCoordinate {
        return SphericalCoordinate::new(alt + EARTH_MSL_RADIUS,
                                        f64::to_radians(lon) + PI,
                                        f64::to_radians(lat) + HALF_PI);
    }

    /// Create a new SphericalCoordinate from cartesian coordinate.
    ///
    /// Scale of vector v needs to be in meters, with reference position being the centre of
    /// the sphere.

    pub fn from_cartesian(v: Vector3<f64>) -> SphericalCoordinate {
        let r = f64::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
        let mut theta = NAN;

        if v.x > 0.0 {
            theta = f64::atan(v.y / v.x);
        } else if v.x < 0.0 {
            if v.y > 0.0 {
                theta = f64::atan(v.y / v.x) + PI;
            } else if v.y < 0.0 {
                theta = f64::atan(v.y / v.x) - PI;
            } else {
                theta = PI;
            }
        } else {
            if v.y > 0.0 {
                theta = PI;
            } else if v.y < 0.0 {
                theta = -PI;
            }
            // else theta = NAN by default
        }

        let phi = f64::acos(v.z / r);

        return SphericalCoordinate::new(r, theta, phi);
    }

    /// Create a clone with values that fall within the normal
    /// boundary of Spherical coordinate system.
    pub fn rectify_bounds(&self) -> SphericalCoordinate {
        let mut new_coord = self.clone();
        new_coord.rectify_bounds_inplace();
        return new_coord;
    }

    /// Ensure that this object's values fall within the normal
    /// boundary of Spherical coordinate system.
    pub fn rectify_bounds_inplace(&mut self) {
        while self.phi < 0.0 {
            self.phi += TWO_PI;
        }

        while self.phi > TWO_PI {
            self.phi -= TWO_PI;
        }

        if self.phi > PI {
            self.phi = TWO_PI - self.phi;
            self.theta += PI;
        } else {
            if self.phi < 0.0 {
                self.phi = -(TWO_PI + self.phi);
                self.theta += PI;
            }
        }

        while self.theta > TWO_PI {
            self.theta -= TWO_PI;
        }

        while self.theta < 0.0 {
            self.theta += TWO_PI;
        }
    }

    /// Check whether this `SphericalCoordinate` and another are approximately equal
    ///
    /// # Examples
    ///
    /// ```
    /// # use oldnav_lib::navdata::coord::SphericalCoordinate;
    /// # use std::f64::consts::PI;
    ///
    /// let pos1 = SphericalCoordinate::new(10.0, PI, 0.0);
    /// let pos2 = SphericalCoordinate::new(10.0, PI, 0.1);
    /// assert!(pos1.approx_eq(&pos2, 0.2));
    ///
    /// ```
    pub fn approx_eq(&self, other: &SphericalCoordinate, epsilon: f64) -> bool {
        let mut eq = true;
        eq &= (self.r - other.r).abs() < epsilon;
        eq &= (self.theta - other.theta).abs() < epsilon;
        eq &= (self.phi - other.phi).abs() < epsilon;
        return eq;
    }

    /// get the altitude
    pub fn alt(&self) -> f64 {
        return self.r - EARTH_MSL_RADIUS;
    }

    /// set the altitude above MSL (in metres)
    pub fn set_alt(&mut self, alt: f64) {
        self.r = alt + EARTH_MSL_RADIUS;
    }

    /// get the latitude (in degrees)
    pub fn lat(&self) -> f64 {
        return (self.phi - HALF_PI).to_degrees();
    }

    /// set the latitude (in degrees)
    pub fn set_lat(&mut self, lat: f64) {
        self.phi = lat.to_radians() + HALF_PI;
        self.rectify_bounds_inplace();
    }

    /// get the longitude (in degrees)
    pub fn lon(&self) -> f64 {
        return (self.theta - PI).to_degrees();
    }

    /// set the longitude (in degrees)
    pub fn set_lon(&mut self, lon: f64) {
        self.theta = lon.to_radians() + PI;
        self.rectify_bounds_inplace();
    }

    /// get the r cartesian unit vector
    pub fn r_cart_uv(&self) -> Vector3<f64> {
        return Vector3 {
            x: self.theta.cos() * self.phi.sin(),
            y: self.theta.sin() * self.phi.sin(),
            z: self.phi.cos(),
        };
    }

    /// get the phi cartesian unit vector
    pub fn phi_cart_uv(&self) -> Vector3<f64> {
        return Vector3 {
            x: self.phi.cos() * self.theta.cos(),
            y: self.phi.cos() * self.theta.sin(),
            z: -self.phi.sin(),
        };
    }

    /// get the theta cartesian unit vector
    pub fn theta_cart_uv(&self) -> Vector3<f64> {
        return Vector3 {
            x: -self.theta.sin(),
            y: self.theta.cos(),
            z: 0.0,
        };
    }

    /// arc distance between two points along the surface of the sphere.
    /// **warning: only tested to be accurate to within 5 meters at earth's surface**
    pub fn arc_distance(&self, other: &SphericalCoordinate) -> f64 {
        // if greater accuracy is required, might be worth checking out the haversine formula
        // or this: https://goo.gl/Niyn91
        return self.r * (self.r_cart_uv().dot(&other.r_cart_uv()).acos());    
    }

    /// Format the `SphericalCoordinate` as a Geographical point string (altitude,
    /// latitude and longitude).
    pub fn fmt_geographic(&self) -> String {
        format!("Point {{alt: {}, lat: {}, lon: {}}}",
                self.alt(),
                self.lat(),
                self.lon())
    }

    /// A very simple difference in angle heuristic
    pub fn angle_difference_heuristic(&self, other: &SphericalCoordinate) -> f64 {
        let dtheta = other.theta - self.theta;
        let dphi = other.phi - self.phi;
        return f64::sqrt(dtheta*dtheta + dphi*dphi);
    }
}

impl fmt::Debug for SphericalCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "SphericalCoordinate {{r: {}, theta: {}, phi: {}}}",
               self.r,
               self.theta,
               self.phi)
    }
}


impl fmt::Display for SphericalCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "SphericalCoordinate {{r: {}, theta: {}, phi: {}}}",
               self.r,
               self.theta,
               self.phi)
    }
}

impl geohash::Geohashable<SphericalCoordinate> for SphericalCoordinate {
    fn integer_decode(geohash: u64) -> Result<SphericalCoordinate, String> {
        let bounds = try!(geohash::decode(geohash, &geohash::LATLON_BOUNDS));
        let pos = bounds.mid();
        let coord = SphericalCoordinate::from_geographic(0.0, pos.y, pos.x);
        return Ok(coord);
    }
    fn integer_encode(&self, precision: u8) -> Result<u64, String> {
        return geohash::encode(
            &Vector2{ x: self.lon(), y: self.lat()},
            precision,
            &geohash::LATLON_BOUNDS);
    }
}


/// Convert degrees minutes seconds format into seconds.
/// 
/// Examples:
/// 
/// ```
/// # use oldnav_lib::navdata::coord::dms_to_deg;
/// let accuracy = 0.0001;
/// let deg = dms_to_deg(10.0, 4.0, 6.0);
/// assert!((deg - 10.06833).abs() < accuracy);
/// ```
pub fn dms_to_deg(degrees: f64, minutes: f64, seconds: f64) -> f64 {
    return degrees + minutes/60.0 + seconds/3600.0;
}
