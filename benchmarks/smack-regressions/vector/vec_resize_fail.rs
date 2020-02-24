// @flag --no-memory-splitting --unroll=3
// @expect error

pub fn main() {
    let mut v1: Vec<u64> = vec![0];
    let mut v2: Vec<u64> = vec![3];
    v1.append(&mut v2);
    verifier::assert_ne!(v1[1], 3);
}
