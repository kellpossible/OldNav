//! An integer based geohash. Inspired by the geohashrust package.
//!
//! It uses the first 4 bits of the u64 to store the precision
//! then the subsequent bits are used to store the hash value.
//!
//! Currently there is a maximum precision of 15
//!

// todo: keep implementing stuff
// geohashrust/geohash.rs.html
// http://gis.stackexchange.com/questions/18330/would-it-be-possible-to-use-geohash-for-proximity-searches

use nalgebra::{Vector2};

// first 4 bits is currently reserved for the precision with a range of 1 to 16

/// maximum value for precision of a geohash
pub static PRECISION_MAX: u8 = 58;

/// minimum value for precision of a geohash
pub static PRECISION_MIN: u8 = 1;

/// number of precision bits
pub static PRECISION_BITS: u8 = 6;

static PRECISION_MASK: u64 = 63; //mask 111111 for the 6 precision bits

/// An object which is able to be geohashed.
pub trait Geohashable<T> {
    /// decode this object of type `T`from an integer geohash
    fn integer_decode(geohash: u64) -> Result<T, String>;

    /// encode this object of type `T` into an unsigned integer geohash
    fn integer_encode(&self, precision: u8) -> Result<u64, String>;
}

/// A trait for a vector type object that can be rectified to fit
/// within a boundary.
pub trait Rectifiable {
    /// wrap this position around the bounds,
    /// such that it becomes the equivalent position
    /// within the bounds.
    fn spherical_rectify(&mut self, range: &Bounds);
}


// todo: figure out a way to implement this for coord so we don't have code and test duplication
impl Rectifiable for Vector2<f64> {
    /// wrap this position around the bounds,
    /// such that it becomes the equivalent position
    /// within the bounds.
    ///
    /// # Example
    ///
    /// ```
    /// #[macro_use]
    /// extern crate nalgebra;
    /// # extern crate oldnav_lib;
    /// # use oldnav_lib::navdata::geohash::*;
    /// use nalgebra::{Vector2, ApproxEq};
    /// # fn main() {
    /// let mut p = Vector2{x: 185.0, y: 30.0};
    /// p.spherical_rectify(&LATLON_BOUNDS);
    /// assert_approx_eq_eps!(p, Vector2{x: -175.0, y: 30.0}, 0.1);
    /// # }
    /// ```
    fn spherical_rectify(&mut self, range: &Bounds) {
        let x_range = range.x_range();
        let y_range = range.y_range();

        println!("x_range = {:#?}", x_range);
        println!("y_range = {:#?}", y_range);

        println!("self.y 0 = {:?}", self.y);

        while self.y < range.y_min - y_range {
            self.y += y_range;
        }

        println!("self.y 1 = {:#?}", self.y);

        while self.y > range.y_max + y_range {
            self.y -= y_range;
        }

        println!("self.y 2 = {:#?}", self.y);
        println!("self.x 1 = {:#?}", self.x);

        if self.y > range.y_max {
            self.y = y_range - self.y;
            self.x += x_range/2.0;
        } else {
            if self.y < range.y_min {
                self.y = -(self.y + y_range);
                self.x += x_range/2.0;
            }
        }

        while self.x > range.x_max {
            self.x -= x_range;
        }

        println!("self.x 3 = {:#?}", self.x);

        while self.x < range.x_min {
            self.x += x_range;
        }

        println!("self.x 4 = {:#?}", self.x);
    }
}

/// A rectangular boundary
#[derive(Clone, Debug, Copy)]
pub struct Bounds {
    /// x coordinate minimum
    pub x_min: f64,

    /// x coordinate maximum
    pub x_max: f64,

    /// y coordinate minimum
    pub y_min: f64,

    /// y coordinate maximum
    pub y_max: f64
}

impl Bounds {
    /// Constructor for `Bounds`
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Bounds
    {
        Bounds {
            x_min: x_min,
            x_max: x_max,
            y_min: y_min,
            y_max: y_max,
        }
    }

    /// Get the midpoint of this `Bound`
    pub fn mid(&self) -> Vector2<f64> {
        Vector2{
            x: (self.x_max + self.x_min)/2.0,
            y: (self.y_max + self.y_min)/2.0
        }
    }

    /// Test whether a `Vector2` position falls within this `Bound`
    ///
    /// # Example
    ///
    /// A point which falls within a boundary:
    ///
    /// ```
    /// # extern crate nalgebra;
    /// # extern crate oldnav_lib;
    /// # use oldnav_lib::navdata::geohash::*;
    /// use nalgebra::Vector2;
    /// # fn main() {
    /// let b = Bounds::new(-21.0, 0.0, 2.0, 4.0);
    /// let p1 = Vector2{x: -2.1, y: 2.45};
    ///
    /// assert_eq!(true, b.contains(&p1));
    /// # }
    /// ```
    ///
    /// A point that falls outside a boundary:
    ///
    /// ```
    /// # extern crate nalgebra;
    /// # extern crate oldnav_lib;
    /// # use oldnav_lib::navdata::geohash::*;
    /// use nalgebra::Vector2;
    /// # fn main() {
    /// let b = Bounds::new(-21.0, 0.0, 2.0, 4.0);
    /// let p2: Vector2<f64> = Vector2{x: -100.0, y: 2.45};
    ///
    /// assert_eq!(false, b.contains(&p2));
    /// # }
    /// ```
    pub fn contains(&self, position: &Vector2<f64>) -> bool {
        if position.x <= self.x_max &&
            position.x >= self.x_min &&
            position.y <= self.y_max &&
            position.y >= self.y_min {
            return true;
        }
        return false;
    }

    /// get the width of the boundary
    pub fn x_range(&self) -> f64 {
        return (self.x_max - self.x_min).abs();
    }

    /// get the height of the boundary
    pub fn y_range(&self) -> f64 {
        return (self.y_max - self.y_min).abs();
    }
}

/// bounds for the lat/lon coordinate system
pub static LATLON_BOUNDS: Bounds = Bounds {
    y_min: -90.0,
    y_max: 90.0,
    x_min: -180.0,
    x_max: 180.0,
};


/// encode a `Vector2` position into an unsigned integer geohash.
/// wraps the position around the boundaries, to keep it within them.
///
/// # Example
///
/// ```
/// # extern crate nalgebra;
/// # extern crate oldnav_lib;
/// # use oldnav_lib::navdata::geohash::*;
/// use nalgebra::Vector2;
/// # fn main() {
/// let p = Vector2{y: 31.23, x: 121.473};
/// let bounds = LATLON_BOUNDS.clone();
/// let gh = encode(&p, 8, &bounds).unwrap();
/// assert_eq!(hash_to_string(gh).unwrap(), "11100110");
/// # }
/// ```
pub fn encode(position: &Vector2<f64>, precision: u8, range: &Bounds) -> Result<u64, String> {
    if precision > PRECISION_MAX || precision < PRECISION_MIN {
        return Err(format!("Precision must be in the range of {} to {}", PRECISION_MIN, PRECISION_MAX));
    }

    let mut current_range: Bounds = range.clone();

    let mut hash: u64 = (precision as u64)-1;

    let mut do_x = true;

    // start leaving room for the precision bits
    let mut i = PRECISION_BITS;
    let end = precision+PRECISION_BITS;
    while i < end {
        let mid = current_range.mid();

        if do_x {
            if position.x > mid.x {
                hash |= 1<<i;
                current_range.x_min = mid.x;
            } else {
                current_range.x_max = mid.x;
            }
        } else {
            if position.y > mid.y {
                hash |= 1<<i;
                current_range.y_min = mid.y;
            } else {
                current_range.y_max = mid.y;
            }
        }

        do_x = !do_x;
        i += 1;
    }

    return Ok(hash);
}

/// Convert an integer geohash to a string representation of the binary values
///
/// # Example
///
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let gh = hash_from_string("11101011").unwrap();
/// assert_eq!("11101011", hash_to_string(gh).unwrap());
/// ```
///
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let gh = hash_from_string("1111").unwrap();
/// assert_eq!("1111", hash_to_string(gh).unwrap());
/// ```
///
pub fn hash_to_string(geohash: u64) -> Result<String, String> {
    let precision: u8 = hash_precision(geohash);
    let mut string = String::with_capacity(precision as usize);

    println!("Precision {}", precision);

    let mut i = PRECISION_BITS;
    let end = PRECISION_BITS+precision;
    while i < end {
        let val = geohash & 1<<i;
        string.push(if val>0 {'1'} else {'0'});
        i += 1;
    }

    return Ok(string);
}

/// Create an integer geohash from a String
///
/// # Example
///
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let gh = hash_from_string("1111").unwrap();
/// assert_eq!(4, hash_precision(gh));
/// assert_eq!("1111", hash_to_string(gh).unwrap());
/// ```
pub fn hash_from_string(string: &str) -> Result<u64, String> {
    let precision = string.len();

    if precision > (PRECISION_MAX as usize) || precision < (PRECISION_MIN as usize) {
        return Err(
            format!("String too long, its length must be in the range of {} to {}",
                    PRECISION_MIN,
                    PRECISION_MAX));
    }

    let mut hash: u64 = (precision as u64)-1;

    let mut i = PRECISION_BITS;

    for char in string.chars() {
        if char == '1' {
            hash |= 1<<i;
        }
        i += 1;
    }

    return Ok(hash);
}

/// Decode integer geohash into a `Bounds`
///
/// # Example
///
/// ```
/// # extern crate nalgebra;
/// # extern crate oldnav_lib;
/// # use oldnav_lib::navdata::geohash::*;
/// use nalgebra::Vector2;
/// # fn main() {
/// let p = Vector2{y: -38.12, x: 148.234};
/// let gh = encode(&p, 40, &LATLON_BOUNDS).unwrap();
/// let b = decode(gh, &LATLON_BOUNDS).unwrap();
/// assert_eq!(true, b.contains(&p));
/// # }
/// ```
pub fn decode(geohash: u64, range: &Bounds) -> Result<Bounds, String> {
    let precision: u8 = hash_precision(geohash);
    return decode_precision_nocheck(geohash, precision, range);
}

/// decode an integer geohash with the specified precision.
pub fn decode_precision(geohash: u64, precision: u8, range: &Bounds) -> Result<Bounds, String> {
    let hp: u8 = hash_precision(geohash);
    if precision > hp {
        return Err(format!(
            "Selected precision ({}) is greater than the max precision of the hash: ({})",
            precision,
            hp
        ));
    }

    return decode_precision_nocheck(geohash, precision, range);
}

/// decode an integer geohash with specified precision, without checking
/// whether or not the precision is less than the hash's internal precision.
pub fn decode_precision_nocheck(geohash: u64, precision: u8, range: &Bounds) -> Result<Bounds, String> {
    let mut current_range: Bounds = range.clone();

    let mut do_x = true;


    let mut i = PRECISION_BITS;
    let end = PRECISION_BITS+precision;

    while i < end {
        let mid = current_range.mid();

        if do_x {
            if geohash & 1<<i > 0 {
                current_range.x_min = mid.x;
            } else {
                current_range.x_max = mid.x;
            }
        } else {
            if geohash & 1<<i > 0 {
                current_range.y_min = mid.y;
            } else {
                current_range.y_max = mid.y;
            }
        }

        do_x = !do_x;
        i += 1;
    }

    return Ok(current_range);
}


/// Get the precision value for an integer geohash.
///
/// # Example
///
/// length of 4:
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let gh = hash_from_string("1010").unwrap();
/// assert_eq!(4, hash_precision(gh));
/// ```
///
/// length of 8:
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let gh = hash_from_string("00000000");
/// assert_eq!(8, hash_precision(gh));
/// ```
pub fn hash_precision(geohash: u64) -> u8 {
    return ((geohash & PRECISION_MASK) as u8) + 1;
}


/// Get the neighboring hash in the direction specified
/// spherical: is whether or not to use spherical rectification/bounds wrapping.
/// # Example
///
/// ```
///
/// ```
pub fn neighbor(geohash: u64, dir: (i8, i8), range: &Bounds, spherical: bool) -> Result<u64, String> {
    let b: Bounds = try!(decode(geohash, range));
    let precision = hash_precision(geohash);
    let (x_dir, y_dir) = dir;
    let midpoint = b.mid();

    let mut np = Vector2{
        x: midpoint.x + b.x_range()*(x_dir as f64),
        y: midpoint.y + b.y_range()*(y_dir as f64)
    };

    if spherical {
        np.spherical_rectify(range)
    }

    return encode(&np, precision, range);
}