//! An integer based geohash.
//!
//! It uses the first 4 bits of the u64 to store the precision
//! then the subsequent bits are used to store the hash value.
//!
//! Currently there is a maximum precision of 15

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

/// A position to be used for hashing
#[derive(Clone, Debug, Copy)]
pub struct Position {
    /// x coordinate
    pub x: f64,

    /// y coordinate
    pub y: f64
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
    pub fn mid(&self) -> Position {
        Position{
            x: (self.x_max + self.x_min)/2.0,
            y: (self.y_max + self.y_min)/2.0
        }
    }

    /// Test whether a `Position` falls within this `Bound`
    ///
    /// # Example
    ///
    /// A point which falls within a boundary:
    ///
    /// ```
    /// # use oldnav_lib::navdata::geohash::*;
    /// let b = Bounds::new(-21.0, 0.0, 2.0, 4.0);
    /// let p1 = Position{x: -2.1, y: 2.45};
    ///
    /// assert_eq!(true, b.contains(&p1));
    /// ```
    ///
    /// A point that falls outside a boundary:
    ///
    /// ```
    /// # use oldnav_lib::navdata::geohash::*;
    /// let b = Bounds::new(-21.0, 0.0, 2.0, 4.0);
    /// let p2 = Position{x: -100.0, y: 2.45};
    ///
    /// assert_eq!(false, b.contains(&p2));
    /// ```
    pub fn contains(&self, position: &Position) -> bool {
        if position.x <= self.x_max &&
            position.x >= self.x_min &&
            position.y <= self.y_max &&
            position.y >= self.y_min {
            return true;
        }
        return false;
    }
}

/// bounds for the lat/lon coordinate system
pub static LATLON_BOUNDS: Bounds = Bounds {
    y_min: -90.0,
    y_max: 90.0,
    x_min: -180.0,
    x_max: 180.0,
};


/// encode a `Position` into an unsigned integer geohash
///
/// # Example
///
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let p = Position{y: 31.23, x: 121.473};
/// let bounds = LATLON_BOUNDS.clone();
/// let gh = encode(&p, 8, &bounds).unwrap();
/// assert_eq!(hash_to_string(gh).unwrap(), "11100110");
/// ```
pub fn encode(position: &Position, precision: u8, range: &Bounds) -> Result<u64, String> {
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

/// Decode integer geohash into a `Position`
///
/// # Example
///
/// ```
/// # use oldnav_lib::navdata::geohash::*;
/// let p = Position{y: -38.12, x: 148.234};
/// let gh = encode(&p, 40, &LATLON_BOUNDS).unwrap();
/// let b = decode(gh, &LATLON_BOUNDS).unwrap();
/// assert_eq!(true, b.contains(&p));
/// ```
pub fn decode(geohash: u64, range: &Bounds) -> Result<Bounds, String> {
    let precision: u8 = hash_precision(geohash);

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