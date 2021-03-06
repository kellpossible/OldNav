extern crate oldnav_lib;

use oldnav_lib::navdata::coord::SphericalCoordinate;

#[test]
fn test_from_geographic() {
    let accuracy = 0.0001;
    let pos = SphericalCoordinate::from_geographic(0.0, 38.0, 148.0);

    // test to see if values were assigned accurately
    // and can be successfully converted back into alt/lat/lon/
    assert!((pos.alt() - 0.0).abs() < accuracy);
    assert!((pos.lat() - 38.0).abs() < accuracy);
    assert!((pos.lon() - 148.0).abs() < accuracy);
}

#[test]
fn test_arc_distance() {
    let accuracy = 5.0; //5 meters accuracy
    let pos1 = SphericalCoordinate::from_geographic(0.0, 38.0, 148.0);
    let pos2 = SphericalCoordinate::from_geographic(0.0, 38.0, 149.0);

    let d = pos1.arc_distance(&pos2);
    assert!((d - 87620.0).abs() < accuracy);
}
