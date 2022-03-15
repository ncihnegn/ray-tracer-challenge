use crate::sphere::Sphere;
use cgmath::{Point3, Vector3};
use derive_more::Constructor;

#[derive(Constructor, Debug, PartialEq)]
pub struct Computation<T> {
    pub object: Sphere<T>,
    pub t: T,
    pub point: Point3<T>,
    pub eyev: Vector3<T>,
    pub normalv: Vector3<T>,
    pub inside: bool,
}
