pub mod math_util {
    pub fn gcd(mut a: u32, mut b: u32) -> u32 {
        assert!(a != 0 && b != 0);
        while b != 0 {
            let c = a % b;
            a = b;
            b = c;
        }
        a
    }
}