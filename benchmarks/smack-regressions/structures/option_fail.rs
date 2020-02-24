// @expect reachable

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
        Some(c) => verifier::assert_eq!(c, 2),
        None => verifier::unreachable!(),
    };
    let y = verifier::nondet!(2u64);
    let b = safe_div(x, y);
    match b {
        Some(c) => verifier::assert_eq!(c, x / y),
        None => verifier::unreachable!(), // Division by zero should return None
    };
}
