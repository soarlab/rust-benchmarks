// @flag --no-memory-splitting --unroll=4
// @expect verified

pub fn main() {
    let mut sum = 0;
    let b = verifier::nondet!(7u64);
    verifier::assume!(b < 5 && b > 1);
    for i in 0..b as u64 {
        sum += i;
    }
    verifier::assert!(2 * sum == b * (b - 1));
}
