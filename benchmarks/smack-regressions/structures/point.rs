// @expect verified

use std::ops::{Add, AddAssign};

#[derive(PartialEq, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn new(_x: u64, _y: u64) -> Point {
        Point { x: _x, y: _y }
    }
    pub fn get_x(self) -> u64 {
        self.x
    }
    pub fn get_y(self) -> u64 {
        self.y
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn main() {
    let w = verifier::nondet!(1u64);
    let x = verifier::nondet!(2u64);
    let y = verifier::nondet!(3u64);
    let z = verifier::nondet!(4u64);

    verifier::assume!(w <= std::u64::MAX / 2); // avoid overflow
    verifier::assume!(x <= std::u64::MAX / 2); // avoid overflow
    verifier::assume!(y <= std::u64::MAX / 2); // avoid overflow
    verifier::assume!(z <= std::u64::MAX / 2); // avoid overflow

    let a = Point::new(w, x);
    let b = Point::new(y, z);
    let c = a + b;
    verifier::assert!(c == Point::new(w + y, x + z));
}
