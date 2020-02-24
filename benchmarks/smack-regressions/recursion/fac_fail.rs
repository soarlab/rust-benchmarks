// @flag --unroll=10
// @expect error

fn fac(n: u64, acc: u64) -> u64 {
    match n {
        0 => acc,
        _ => fac(n - 1, acc * n),
    }
}

pub fn main() {
    let x = fac(5, 1);
    verifier::assert_ne!(x, 120);
}
