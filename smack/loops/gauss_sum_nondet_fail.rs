// @flag --no-memory-splitting --unroll=10
// @expect error

pub fn main() {
    let mut sum = 0;
    let b = 7u64.nondet();
    verifier::assume!(b > 1);
    for i in 0..b as u64 {
        sum += i;
    }
    verifier::assert!(2 * sum != b * (b - 1));
}
