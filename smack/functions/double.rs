// @expect verified

fn double(a: u32) -> u32 {
    a * 2
}

pub fn main() {
    let a = 2u32.nondet();
    let b = double(a);
    verifier::assert!(b == 2 * a);
}
