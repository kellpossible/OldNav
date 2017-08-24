extern crate oldnav_lib;

#[macro_use]
extern crate approx;

#[macro_use]
extern crate nalgebra;

use oldnav_lib::navdata::geohash::*;
use nalgebra::Vector2;
use approx::ApproxEq;

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out1() {
    let mut p = Vector2::new(-185.0, 30.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(175.0, 30.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out2() {
    let mut p = Vector2::new(5.0, 95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(-175.0, 85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out3() {
    let mut p = Vector2::new(-5.0, 95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(175.0, 85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out4() {
    let mut p = Vector2::new(175.0, 95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(-5.0, 85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out5() {
    let mut p = Vector2::new(175.0, -95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(-5.0, -85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out6() {
    let mut p = Vector2::new(185.0, -95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(5.0, -85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out7() {
    let mut p = Vector2::new(-185.0, -95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(-5.0, -85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out8() {
    let mut p = Vector2::new(185.0, 95.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(5.0, 85.0), epsilon = 0.0001);
}

/// test spherical rectify with coordinate that is inside the boundary
#[test]
fn test_spherical_rectify_in1() {
    let mut p = Vector2::new(5.0, -30.0);
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_relative_eq!(p, Vector2::new(5.0, -30.0), epsilon = 0.0001);
}
