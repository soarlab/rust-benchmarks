// @expect error

fn safe_div(x: u64, y: u64) -> Option<u64> {
    if y != 0 {
        Some(x / y)
    } else {
        None
    }
}

pub fn main() {
    let x = 2u64.nondet();
    verifier::assume!(x > 0);
    let a = safe_div(2 * x, x);
    match a {
        Some(x) => verifier::assert!(x == 2),
        None => verifier::assert!(false),
    };
    let b = safe_div(x, 0);
    match b {
        Some(x) => verifier::assert!(true),
        None => verifier::assert!(false), // Division by zero should return None
    };
}
