use num_traits::Float;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::result::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Point<T: Float> {
    x: T,
    y: T,
    z: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vector<T: Float> {
    x: T,
    y: T,
    z: T,
}

impl<T: Float> Point<T> {
    fn new(x: T, y: T, z: T) -> Point<T> {
        Point::<T> { x, y, z }
    }
}

impl<T: Float> Vector<T> {
    fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector::<T> { x, y, z }
    }

    fn cross(self, rhs: Vector<T>) -> Vector<T> {
        Vector::<T> {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    fn dot(self, rhs: Vector<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self) -> Vector<T> {
        self / self.magnitude()
    }
}

impl<T: Float> Add<Vector<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> Neg for Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Sub for Point<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> Sub<Vector<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Float> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::abs_diff_eq;

    #[test]
    fn add() {
        assert_eq!(
            Vector::<f32>::new(3., -2., 5.,) + Vector::<f32>::new(-2., 3., 1.,),
            Vector::<f32>::new(1., 1., 6.,)
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            Vector::<f32>::new(1., 2., 3.).cross(Vector::<f32>::new(2., 3., 4.)),
            Vector::<f32>::new(-1., 2., -1.)
        );
        assert_eq!(
            Vector::<f32>::new(2., 3., 4.).cross(Vector::<f32>::new(1., 2., 3.)),
            Vector::<f32>::new(1., -2., 1.)
        );
    }

    #[test]
    fn div() {
        assert_eq!(
            Vector::<f32>::new(1., -2., 3.) / 2.,
            Vector::<f32>::new(0.5, -1., 1.5)
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            Vector::<f32>::new(1., 2., 3.).dot(Vector::<f32>::new(2., 3., 4.)),
            20.
        );
    }

    #[test]
    fn magnitude() {
        assert_eq!(
            Vector::<f32>::new(1., -2., 3.).magnitude(),
            (14.0 as f32).sqrt()
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Vector::<f32>::new(1., -2., 3.) * 3.5,
            Vector::<f32>::new(3.5, -7., 10.5)
        );
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Vector::<f32>::new(1., -2., 3.),
            Vector::<f32>::new(-1., 2., -3.)
        );
    }

    #[test]
    fn normalize() {
        abs_diff_eq!(Vector::<f32>::new(1., -2., 3.).normalize().magnitude(), 1.);
    }

    #[test]
    fn sub() {
        assert_eq!(
            Point::<f32>::new(3., 2., 1.) - Point::<f32>::new(5., 6., 7.),
            Vector::<f32>::new(-2., -4., -6.)
        );
        assert_eq!(
            Point::<f32>::new(3., 2., 1.) - Vector::<f32>::new(5., 6., 7.),
            Point::<f32>::new(-2., -4., -6.)
        );
        assert_eq!(
            Vector::<f32>::new(3., 2., 1.) - Vector::<f32>::new(5., 6., 7.),
            Vector::<f32>::new(-2., -4., -6.)
        );
    }
}
