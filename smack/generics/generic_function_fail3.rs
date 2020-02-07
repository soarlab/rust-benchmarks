// @expect error

struct Point<T> {
    pub x: T,
    pub y: T,
}

struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

trait S<T> {
    fn swap_items(self) -> Self;
}

impl<T> S<T> for Point<T> {
    fn swap_items(self) -> Point<T> {
        Point::<T> {
            x: self.y,
            y: self.x,
        }
    }
}

impl<T> S<T> for Point3<T> {
    fn swap_items(self) -> Point3<T> {
        Point3::<T> {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }
}

fn swapem<T, U: S<T>>(s: U) -> U {
    s.swap_items()
}

pub fn main() {
    let x2 = verifier::nondet!(7i64);
    let y2 = verifier::nondet!(8i64);
    let x3 = verifier::nondet!(1i64);
    let y3 = verifier::nondet!(2i64);
    let z3 = verifier::nondet!(3i64);
    let p2 = Point::<i64> { x: x2, y: y2 };
    let p3 = Point3::<i64> {
        x: x3,
        y: y3,
        z: z3,
    };

    let q2 = swapem(p2);
    let q3 = swapem(p3);
    verifier::assert!(q2.x == y2);
    verifier::assert!(q2.y == x2);
    verifier::assert!(q3.x != y3);
    verifier::assert!(q3.y == z3);
    verifier::assert!(q3.z == x3);
}
