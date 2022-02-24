use std::ops::{Add, Div, Mul, Neg, Sub};
use std::result::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

fn point(x: f32, y: f32, z: f32) -> Point {
    Point { x, y, z }
}

fn vector(x: f32, y: f32, z: f32) -> Vector {
    Vector { x, y, z }
}

impl Vector {
    fn cross(self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    fn dot(self, rhs: Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self) -> Result<Vector, String> {
        self / self.magnitude()
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Result<Vector, String>;

    fn div(self, rhs: f32) -> Self::Output {
        if rhs == 0. {
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

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
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

    #[test]
    fn add() {
        assert_eq!(
            Vector {
                x: 3.,
                y: -2.,
                z: 5.,
            } + Vector {
                x: -2.,
                y: 3.,
                z: 1.,
            },
            Vector {
                x: 1.,
                y: 1.,
                z: 6.,
            }
        );
    }

    #[test]
    fn cross() {
        assert_eq!(
            vector(1., 2., 3.).cross(vector(2., 3., 4.)),
            vector(-1., 2., -1.)
        );
        assert_eq!(
            vector(2., 3., 4.).cross(vector(1., 2., 3.)),
            vector(1., -2., 1.)
        );
    }

    #[test]
    fn div() {
        assert_eq!(vector(1., -2., 3.) / 2., Ok(vector(0.5, -1., 1.5)));
    }

    #[test]
    fn dot() {
        assert_eq!(vector(1., 2., 3.).dot(vector(2., 3., 4.)), 20.);
    }

    #[test]
    fn magnitude() {
        assert_eq!(vector(1., -2., 3.).magnitude(), (14.0 as f32).sqrt());
    }

    #[test]
    fn mul() {
        assert_eq!(vector(1., -2., 3.) * 3.5, vector(3.5, -7., 10.5));
    }

    #[test]
    fn neg() {
        assert_eq!(-vector(1., -2., 3.), vector(-1., 2., -3.));
    }

    #[test]
    fn normalize() {
        abs_diff_eq!(vector(1., -2., 3.).normalize().unwrap().magnitude(), 1.);
    }

    #[test]
    fn sub() {
        assert_eq!(point(3., 2., 1.) - point(5., 6., 7.), vector(-2., -4., -6.));
        assert_eq!(point(3., 2., 1.) - vector(5., 6., 7.), point(-2., -4., -6.));
        assert_eq!(
            vector(3., 2., 1.) - vector(5., 6., 7.),
            vector(-2., -4., -6.)
        );
    }
}
