extern crate rs_common;
use rs_common::math_util::{gcd, ceil, floor};

#[test]
fn test_gcd() {
    assert_eq!(gcd(8, 12), 4);
    assert_eq!(gcd(12, 18), 6);
}

#[test]
fn test_ceil_and_floor() {
    assert_eq!(ceil(567.12), 568);
    assert_eq!(ceil(-100000000000.12), -100000000000);
    assert_eq!(floor(567.12), 567);
    assert_eq!(floor(-100000000000.12), -100000000001);
}