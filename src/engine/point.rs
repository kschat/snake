use std::ops;

use impl_ops::*;
use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point<T = usize>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn unit() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl From<&Point<usize>> for (usize, usize) {
    fn from(value: &Point<usize>) -> Self {
        (value.x, value.y)
    }
}

pub type Vector = Point<isize>;

impl_op_ex!(+|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});

impl_op_ex!(-|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(*|a: &Point, b: &Point| -> Point {
    Point {
        x: a.x * b.x,
        y: a.y * b.y,
    }
});

impl_op_ex_commutative!(*|a: &Point, b: &usize| -> Point {
    Point {
        x: a.x * b,
        y: a.y * b,
    }
});

impl_op_ex!(+|a: &Point, b: &Vector| -> Point {
    Point {
        x: (a.x as isize + b.x) as usize,
        y: (a.y as isize + b.y) as usize,
    }
});
