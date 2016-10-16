extern crate oldnav_lib;

#[macro_use]
extern crate nalgebra;

use oldnav_lib::navdata::geohash::*;
use nalgebra::{Vector2, ApproxEq};

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out1() {
    let mut p = Vector2{x: -185.0, y: 30.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: 175.0, y: 30.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out2() {
    let mut p = Vector2{x: 5.0, y: 95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: -175.0, y: 85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out3() {
    let mut p = Vector2{x: -5.0, y: 95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: 175.0, y: 85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out4() {
    let mut p = Vector2{x: 175.0, y: 95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: -5.0, y: 85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out5() {
    let mut p = Vector2{x: 175.0, y: -95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: -5.0, y: -85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out6() {
    let mut p = Vector2{x: 185.0, y: -95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: 5.0, y: -85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out7() {
    let mut p = Vector2{x: -185.0, y: -95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: -5.0, y: -85.0}, 0.0001);
}

/// test spherical rectify with coordinate that falls outside the boundary
#[test]
fn test_spherical_rectify_out8() {
    let mut p = Vector2{x: 185.0, y: 95.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: 5.0, y: 85.0}, 0.0001);
}

/// test spherical rectify with coordinate that is inside the boundary
#[test]
fn test_spherical_rectify_in1() {
    let mut p = Vector2{x: 5.0, y: -30.0};
    p.spherical_rectify(&LATLON_BOUNDS);
    assert_approx_eq_eps!(p, Vector2{x: 5.0, y: -30.0}, 0.0001);
}