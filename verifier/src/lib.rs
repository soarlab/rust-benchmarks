#![feature(panic_info_message)]

use core::panic::PanicInfo;
use std::os::raw;
use std::ffi::CString;
use std::default::Default;
use std::fmt::Write;

pub fn verifier_assume(cond: bool) {
    extern "C" { fn klee_assume(cond: usize); }
    unsafe { klee_assume(if cond {1} else {0}) }
}

pub fn verifier_verify(cond: bool) {
    if !cond {
        report_error("verification failed")
    }
}

pub fn verifier_abstract_value<T: Default>(_t: T) -> T {
    extern "C" { fn klee_make_symbolic(data: *mut raw::c_void, length: usize, name: *const raw::c_char); }

    let mut r = T::default();
    unsafe {
        let data   = std::mem::transmute(&mut r);
        let length = std::mem::size_of::<T>();
        let null = 0 as *const i8;
        klee_make_symbolic(data, length, null)
    }
    return r;
}

extern "C" {
    fn klee_report_error(file: *const raw::c_char, line: usize, message: *const raw::c_char, suffix: *const raw::c_char) -> !;
}

pub fn report_error(message: &str) -> ! {
    let null = 0 as *const i8;
    let file = null; // ignored by KLEE
    let line = 0;    // ignored by KLEE
    let suffix = ""; // ignored by KLEE

    unsafe {
        klee_report_error(file,
                          line,
                          CString::new(message).unwrap().as_ptr(),
                          CString::new(suffix).unwrap().as_ptr())
    }
}

fn panic_hook(info: &PanicInfo) {
    let mut msg = String::new();
    match info.message() {
        None => msg.write_str("panic"),
        Some(m) => msg.write_fmt(*m)
    }.unwrap();

    let null = 0 as *const i8;
    let file = null; // ignored by KLEE
    let line = 0;    // ignored by KLEE
    let suffix = ""; // ignored by KLEE

    unsafe {
        klee_report_error(file,
                          line,
                          CString::new(msg).unwrap().as_ptr(),
                          CString::new(suffix).unwrap().as_ptr())
    }
}

// Calling this before starting verification ensures that
// panic messages are displayed by KLEE.
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(panic_hook))
}


#[macro_export]
macro_rules! assume {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::verifier_assume($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($condition:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::verifier_verify($condition)
        } else {
            assert!($condition);
        }
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            $crate::verifier_verify($left == $right)
        } else {
            assert_eq!($left, $right);
        }
    );
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr) => (
        if cfg!(feature = "verifier-klee") {
            $crate::verifier_verify($left != $right)
        } else {
            assert!($left != $right);
        }
    );
}

#[macro_export]
macro_rules! unreachable {
    () => (
        if cfg!(feature = "verifier-klee") {
            $crate::report_error("unreachable");
        } else {
            unreachable!();
        }
    );
}

#[macro_export]
macro_rules! nondet {
    ($value:expr) => {
        if cfg!(feature = "verifier-klee") {
            $crate::verifier_abstract_value($value)
        } else {
            $value
        }
    };
}
