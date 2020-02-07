// @expect error

fn double(a: u32) -> u32 {
    a * 2
}

pub fn main() {
    let a = 2u32.nondet();
    let b = double(a);
    verifier::verifier_assert!(b != 2 * a);
}
