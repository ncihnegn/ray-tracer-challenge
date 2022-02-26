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

fn point<T: Float>(x: T, y: T, z: T) -> Point<T> {
    Point::<T> { x, y, z }
}

fn vector<T: Float>(x: T, y: T, z: T) -> Vector<T> {
    Vector::<T> { x, y, z }
}

impl<T: Float> Vector<T> {
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

    fn normalize(self) -> Result<Vector<T>, String> {
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
    type Output = Result<Vector<T>, String>;

    fn div(self, rhs: T) -> Self::Output {
        if rhs == T::zero() {
            Err("Divided by zero.".to_string())
        } else {
            Ok(Vector {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            })
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
            vector::<f32>(3., -2., 5.,) + vector::<f32>(-2., 3., 1.,),
            vector::<f32>(1., 1., 6.,)
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            vector::<f32>(1., 2., 3.).cross(vector::<f32>(2., 3., 4.)),
            vector::<f32>(-1., 2., -1.)
        );
        assert_eq!(
            vector::<f32>(2., 3., 4.).cross(vector::<f32>(1., 2., 3.)),
            vector::<f32>(1., -2., 1.)
        );
    }

    #[test]
    fn div() {
        assert_eq!(
            vector::<f32>(1., -2., 3.) / 2.,
            Ok(vector::<f32>(0.5, -1., 1.5))
        );
    }

    #[test]
    fn dot() {
        assert_eq!(
            vector::<f32>(1., 2., 3.).dot(vector::<f32>(2., 3., 4.)),
            20.
        );
    }

    #[test]
    fn magnitude() {
        assert_eq!(vector::<f32>(1., -2., 3.).magnitude(), (14.0 as f32).sqrt());
    }

    #[test]
    fn mul() {
        assert_eq!(
            vector::<f32>(1., -2., 3.) * 3.5,
            vector::<f32>(3.5, -7., 10.5)
        );
    }

    #[test]
    fn neg() {
        assert_eq!(-vector::<f32>(1., -2., 3.), vector::<f32>(-1., 2., -3.));
    }

    #[test]
    fn normalize() {
        abs_diff_eq!(
            vector::<f32>(1., -2., 3.).normalize().unwrap().magnitude(),
            1.
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            point::<f32>(3., 2., 1.) - point::<f32>(5., 6., 7.),
            vector::<f32>(-2., -4., -6.)
        );
        assert_eq!(
            point::<f32>(3., 2., 1.) - vector::<f32>(5., 6., 7.),
            point::<f32>(-2., -4., -6.)
        );
        assert_eq!(
            vector::<f32>(3., 2., 1.) - vector::<f32>(5., 6., 7.),
            vector::<f32>(-2., -4., -6.)
        );
    }
}
