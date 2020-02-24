// @expect verified
// @flag --bit-precise

pub fn main() {
    // unsigned
    {
        let a: u32 = 2;
        let b: u32 = 3;
        {
            let c = a + b;
            verifier::assert_eq!(c, 5);
        }
        {
            let c = a * b;
            verifier::assert_eq!(c, 6);
        }
        {
            let c = b - a;
            verifier::assert_eq!(c, 1);
        }
        {
            let c = a % b;
            verifier::assert_eq!(c, 2);
            let d = b % a;
            verifier::assert_eq!(d, 1);
        }
        {
            let c = a / b;
            verifier::assert_eq!(c, 0);
            let d = b / a;
            verifier::assert_eq!(d, 1);
        }
    }
    // signed
    {
        let a: i32 = -3;
        let b: i32 = 5;
        {
            let c = a + b;
            verifier::assert_eq!(c, 2);
        }
        {
            let c = a * b;
            verifier::assert_eq!(c, -15);
        }
        {
            let c = b - a;
            verifier::assert_eq!(c, 8);
        }
    }
}
