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

