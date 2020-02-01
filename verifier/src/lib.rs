
#[macro_export]
macro_rules! verify_assume {
    ($condition:expr) => {
        if cfg!(mirai) {
            mirai_annotations::mirai_assume($condition)
        }
    };
}

#[macro_export]
macro_rules! verifier_assert {
    ($condition:expr) => {
        if cfg!(mirai) {
            mirai_annotations::mirai_verify($condition, "false verification condition")
        }
    };
}

