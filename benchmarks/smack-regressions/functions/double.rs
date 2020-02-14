// @expect verified

fn double(a: u32) -> u32 {
    a * 2
}

pub fn main() {
    let a = verifier::nondet!(2u32);
    verifier::assume!(a <= std::u32::MAX / 2); // avoid overflow
    let b = double(a);
    verifier::assert!(b == 2 * a);
}
