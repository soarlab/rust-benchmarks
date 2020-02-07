// @expect error

pub fn main() {
    let a = verifier::nondet!(6i32);
    let b = verifier::nondet!(7i32);
    verifier::assume!(4 < a && a < 8); // a in [5,7]
    verifier::assume!(5 < b && b < 9); // b in [6,8]
    let x = a * b;
    verifier::assert!(
        !(x == 30 || x == 35 || x == 40 || x == 36 || x == 48 || x == 42 || x == 49 || x == 56)
    ); // a*b != anything allowed
}
