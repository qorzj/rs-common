use crate::math_util::math_util::gcd;

mod math_util;

fn main() {
    let a = gcd(4, 11);
    println!("{}", a);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(8, 12), 4);
    assert_eq!(gcd(24, 18), 6);
}