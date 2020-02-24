// @flag --no-memory-splitting
// @expect error

pub fn main() {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v.push(3);
    verifier::assert_eq!(v[0], 0);
    verifier::assert_eq!(v[1], 1);
    verifier::assert_eq!(v[2], 3);
    v[2] = v[0] + v[1];
    verifier::assert_eq!(v[0], 0);
    verifier::assert_ne!(v[1], 1);
    verifier::assert_eq!(v[2], 1);
}
