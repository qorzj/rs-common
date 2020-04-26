extern crate rs_common;
use rs_common::str_util::to_fixed;

#[test]
fn test_to_fixed() {
    assert_eq!(to_fixed(12.3456, 2), "12.35");
    assert_eq!(to_fixed(12.345, 2), "12.35");
    assert_eq!(to_fixed(12.355, 2), "12.36");
    assert_eq!(to_fixed(12., 2), "12");
}