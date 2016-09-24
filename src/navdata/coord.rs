use nalgebra::Vector3;
use std::f64::consts::PI;
use std::f64::*;
use std::fmt;

static TWO_PI: f64 = PI * 2.0;
static HALF_PI: f64 = PI / 2.0;

pub static EARTH_MSL_RADIUS: f64 = 6371000.0;

pub struct SphericalCoordinate {
    r: f64,
    theta: f64,
    phi: f64,
}


impl SphericalCoordinate {
    pub fn new(r: f64, theta: f64, phi: f64) -> SphericalCoordinate {
        return SphericalCoordinate {
            r: r,
            theta: theta,
            phi: phi,
        };
    }

    /// Create a new SphericalCoordinate from geographic coordinate
    pub fn from_geographic(alt: f64, lat: f64, lon: f64) -> SphericalCoordinate {
        return SphericalCoordinate::new(alt + EARTH_MSL_RADIUS,
                                        f64::to_radians(lon) + PI,
                                        f64::to_radians(lat) + HALF_PI);
    }

    /// Create a new SphericalCoordinate from cartesian coordinate
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

        return SphericalCoordinate::new(r, theta, phi).rectify_bounds();
    }

    fn rectify_bounds(&self) -> SphericalCoordinate {
        let mut r = self.r;
        let mut theta = self.theta;
        let mut phi = self.phi;
        while phi < 0.0 {
            phi += TWO_PI;
        }

        while phi > TWO_PI {
            phi -= TWO_PI;
        }

        if phi > PI {
            phi = TWO_PI - phi;
            theta += PI;
        }

        if theta < 0.0 {
            r = -r;
            theta += PI;
            phi = PI - phi;
        }

        while theta > TWO_PI {
            theta -= TWO_PI;
        }

        while theta < 0.0 {
            theta += TWO_PI;
        }

        return SphericalCoordinate::new(r, theta, phi);
    }

    /// get the altitude
    pub fn alt(&self) -> f64 {
        return self.r - EARTH_MSL_RADIUS;
    }

    /// set the altitude above MSL
    pub fn set_alt(&mut self, alt: f64) {
        self.r = alt + EARTH_MSL_RADIUS;
    }

    /// get the latitude
    pub fn lat(&self) -> f64 {
        return (self.phi - HALF_PI).to_degrees();
    }

    /// set the latitude
    pub fn set_lat(&mut self, lat: f64) {
        self.phi = lat.to_radians() + HALF_PI;
    }

    /// get the longitude
    pub fn lon(&self) -> f64 {
        return (self.theta - PI).to_degrees();
    }


    /// set the longitude
    pub fn set_lon(&mut self, lon: f64) {
        self.theta = lon.to_radians() + PI;
    }

    /// get the r cartesian unit vector
    pub fn r_cart_uv(&self) -> Vector3<f64> {
        let rectified = self.rectify_bounds();
        let theta = rectified.theta;
        let phi = rectified.phi;

        return Vector3 {
            x: theta.cos() * phi.sin(),
            y: theta.sin() * phi.sin(),
            z: phi.cos(),
        };
    }

    /// get the phi cartesian unit vector
    pub fn phi_cart_uv(&self) -> Vector3<f64> {
        let rectified = self.rectify_bounds();
        let theta = rectified.theta;
        let phi = rectified.phi;

        return Vector3 {
            x: phi.cos() * theta.cos(),
            y: phi.cos() * theta.sin(),
            z: -phi.sin(),
        };
    }

    /// get the theta cartesian unit vector
    pub fn theta_cart_uv(&self) -> Vector3<f64> {
        let rectified = self.rectify_bounds();
        let theta = rectified.theta;

        return Vector3 {
            x: -theta.sin(),
            y: theta.cos(),
            z: 0.0,
        };
    }

    /// arc distance between two points along the surface of the sphere.
    pub fn arc_distance(&self, other: &SphericalCoordinate) -> f64 {
        return self.r *
               f64::cos(self.theta.cos() * other.theta.cos() +
                        other.theta.sin() * f64::cos(self.phi - other.phi));
    }

    pub fn fmt_geographic(&self) -> String {
        format!("Point {{alt: {}, lat: {}, lon: {}}}",
                self.alt(),
                self.lat(),
                self.lon())
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
