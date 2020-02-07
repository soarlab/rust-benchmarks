// @expect verified

pub fn main() {
    let a = 6i32.nondet();
    let b = 7i32.nondet();
    verifier::assume!(4 < a && a < 8); // a in [5,7]
    verifier::assume!(5 < b && b < 9); // b in [6,8]
    verifier::assert!(30 <= a * b && a * b <= 56); // a*b in [30,56]
}
