// @flag --no-memory-splitting --unroll=10
// @expect error

pub fn main() {
    let mut sum = 0;
    let b = verifier::nondet!(7u64);
    verifier::assume!(b > 1);
    for i in 0..b as u64 {
        sum += i;
    }
    verifier::assert_ne!(2 * sum, b * (b - 1));
}
