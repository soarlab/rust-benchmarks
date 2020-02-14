// @expect verified

fn safe_div(x: u64, y: u64) -> Option<u64> {
    if y != 0 {
        Some(x / y)
    } else {
        None
    }
}

pub fn main() {
    let x = verifier::nondet!(2u64);
    verifier::assume!(x > 0);
    verifier::assume!(x <= std::u64::MAX / 2); // avoid overflow
    let a = safe_div(2 * x, x);
    match a {
        Some(x) => verifier::assert!(x == 2),
        None => verifier::assert!(false),
    };
    let b = safe_div(x, 0);
    match b {
        Some(_x) => verifier::assert!(false),
        None => verifier::assert!(true),
    };
}
