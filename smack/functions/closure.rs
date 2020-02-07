// @flag --no-memory-splitting
// @expect verified

fn call_with_one<F>(mut some_closure: F) -> ()
where
    F: FnMut(i32) -> (),
{
    some_closure(1);
}

pub fn main() {
    let mut num = verifier::nondet!(5i32);
    let original_num = num;
    {
        let mut add_num = |x: i32| num += x;

        add_num(5);
        call_with_one(&mut add_num);
    }
    verifier::assert_eq!(original_num + 6, num);
}
