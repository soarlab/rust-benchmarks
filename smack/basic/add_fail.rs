// @expect error

pub fn main() {
    let a = 2;
    let b = 3;
    verifier::verifier_assert!(a + b == 6);
}
