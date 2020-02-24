#[macro_export]
macro_rules! assume {
    ($condition:expr) => {
        if cfg!(mirai) {
            mirai_annotations::mirai_assume($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($condition:expr) => {
        if cfg!(mirai) {
            mirai_annotations::mirai_verify($condition, "false verification condition")
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
            mirai_annotations::mirai_verify($left == $right, concat!("false verification condition: ", stringify!($left == $right)))
        } else {
            assert_eq!($left, $right);
        }
    );
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => (
        if cfg!(mirai) {
            mirai_annotations::mirai_verify($left != $right, concat!("false verification condition: ", stringify!($left != $right)))
        } else {
            assert_ne!($left, $right);
        }
    );
}

#[macro_export]
macro_rules! unreachable {
    () => (
        if cfg!(mirai) {
            panic!("statement is reachable");
        } else {
            unreachable!();
        }
    );
}

#[macro_export]
macro_rules! nondet {
    ($value:expr) => {
        if cfg!(mirai) {
            mirai_annotations::mirai_abstract_value($value)
        } else {
            $value
        }
    };
}
