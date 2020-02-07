// @flag --no-memory-splitting --unroll=4
// @expect verified

fn fac(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * fac(n - 1),
    }
}

pub fn main() {
    let mut a = 1;
    let n = verifier::nondet!(6u64);
    verifier::assume!(n < 5);
    for i in 1..n + 1 as u64 {
        a *= i;
    }
    verifier::assert!(a == fac(n)); // a == 6!
}
