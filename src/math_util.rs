pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    assert!(a != 0 && b != 0);
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

pub fn ceil(a: f64) -> i64 {
    return a.ceil() as i64;
}

pub fn floor(a: f64) -> i64 {
    return a.floor() as i64;
}