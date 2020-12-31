use std::ops;

use num_traits::Num;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T = f32>
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

// usize for x and y feels wrong, but it's needed to index into the buffer
pub type AbsPoint = Point<usize>;

impl From<Point<f32>> for AbsPoint {
    fn from(point: Point<f32>) -> Self {
        Self {
            x: point.x.floor() as usize,
            y: point.y.floor() as usize,
        }
    }
}

impl_op_ex!(+|a: &Point<f32>, b: &Point<f32>| -> Point<f32> {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});

impl_op_ex!(-|a: &Point<f32>, b: &Point<f32>| -> Point<f32> {
    Point {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});

impl_op_ex!(*|a: &Point<f32>, b: &Point<f32>| -> Point<f32> {
    Point {
        x: a.x * b.x,
        y: a.y * b.y,
    }
});

impl_op_ex_commutative!(*|a: &Point<f32>, b: &f32| -> Point<f32> {
    Point {
        x: a.x * b,
        y: a.y * b,
    }
});

impl_op_ex!(+|a: &Point<usize>, b: &Point<usize>| -> Point<usize> {
    Point {
        x: a.x + b.x,
        y: a.y + b.y,
    }
});
