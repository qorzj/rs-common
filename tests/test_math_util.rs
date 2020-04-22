extern crate rs_common;
use rs_common::math_util::gcd;

#[test]
fn test_gcd() {
    assert_eq!(gcd(8, 12), 4);
    assert_eq!(gcd(12, 18), 6);
}