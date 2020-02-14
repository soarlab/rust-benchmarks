// @expect verified

pub fn main() {
    let a = verifier::nondet!(6i32);
    let b = verifier::nondet!(7i32);
    verifier::assume!(4 < a && a < 8); // a in [5,7]
    verifier::assume!(5 < b && b < 9); // b in [6,8]
    verifier::assert!(30 <= a * b && a * b <= 56); // a*b in [30,56]
}
