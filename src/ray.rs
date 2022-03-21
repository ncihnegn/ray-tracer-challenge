use crate::impl_approx;
use crate::intersection::Intersection;
use crate::shape::{Shape, Sphere};
use cgmath::{
    AbsDiffEq, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, RelativeEq, SquareMatrix,
    UlpsEq, Vector3,
};
use derive_more::Constructor;

#[derive(Constructor, Copy, Clone, Debug, PartialEq)]
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
}

impl_approx!(Ray=> Point3<T> Vector3<T> => origin direction);

impl<T: BaseFloat> Ray<T> {
    // Find the position after time.
    pub fn position(&self, time: T) -> Point3<T> {
        self.origin + self.direction * time
    }

    pub fn transform(&self, transform: Matrix4<T>) -> Ray<T> {
        Ray::<T> {
            origin: Point3::from_homogeneous(transform * self.origin.to_homogeneous()),
            direction: (transform * self.direction.extend(T::zero())).truncate(),
        }
    }
}

mod tests {
    use super::*;
    use crate::material::Material;

    #[test]
    fn position() {
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(0.),
            Point3::new(2., 3., 4.)
        );
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(-1.),
            Point3::new(1., 3., 4.)
        );
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(2.5),
            Point3::new(4.5, 3., 4.)
        );
    }

    #[test]
    fn transform() {
        assert_eq!(
            Ray::new(Point3::new(1., 2., 3.), Vector3::unit_y(),)
                .transform(Matrix4::from_translation(Vector3::new(3., 4., 5.))),
            Ray::new(Point3::new(4., 6., 8.), Vector3::unit_y(),)
        );
        assert_eq!(
            Ray::new(Point3::new(1., 2., 3.), Vector3::unit_y(),)
                .transform(Matrix4::from_nonuniform_scale(2., 3., 4.)),
            Ray::new(Point3::new(2., 6., 12.), Vector3::new(0., 3., 0.),)
        );
    }
}
