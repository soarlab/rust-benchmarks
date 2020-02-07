// @expect error

pub fn main() {
    let a = 2;
    let b = 3;
    verifier::assert!(b % a != 1);
}
