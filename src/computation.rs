use crate::shape::Shape;
use cgmath::{BaseFloat, Point3, Vector3};
use derive_more::Constructor;

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Computation<T> {
    pub object: Shape<T>,
    pub t: T,
    pub point: Point3<T>,
    pub eyev: Vector3<T>,
    pub normalv: Vector3<T>,
    pub inside: bool,
    pub reflectv: Vector3<T>,
}

impl<T: BaseFloat> Computation<T> {
    pub fn over_point(&self) -> Point3<T> {
        self.point + self.normalv * T::epsilon()
    }
}
