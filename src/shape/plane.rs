use crate::{
    bounds::Bounds, intersection::Intersection, material::Material, ray::Ray, shape::Shape,
};
use cgmath::{abs_diff_eq, BaseFloat, Matrix4, Point3, SquareMatrix, Vector3};

#[derive(Clone, derive_more::Constructor, Debug, PartialEq)]
pub struct Plane<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T: BaseFloat + Default> Default for Plane<T> {
    fn default() -> Plane<T> {
        Plane::<T> {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl<T: BaseFloat> Plane<T> {
    pub fn bounds(&self) -> Bounds<T> {
        let max = Point3::new(T::max_value(), T::zero(), T::max_value());
        let min = Point3::new(T::min_value(), T::zero(), T::min_value());
        Bounds::new(min, max)
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if abs_diff_eq!(ray.direction.y, T::zero()) {
            Vec::new()
        } else {
            vec![Intersection::new(
                -ray.origin.y / ray.direction.y,
                Shape::Plane(self.clone()),
                None,
            )]
        }
    }

    pub fn local_normal_at(&self, _point: Point3<T>) -> Vector3<T> {
        Vector3::unit_y()
    }
}

mod tests {
    use super::*;
    use cgmath::{assert_relative_eq, EuclideanSpace};

    #[test]
    fn local_intersect() {
        let plane = Plane::default();
        let shape = Shape::Plane(plane.clone());
        let vy = Vector3::unit_y();
        let v = vec![Intersection::new(1., shape, None)];
        assert_eq!(
            plane.local_intersect(Ray::new(Point3::from_vec(vy), -vy)),
            v
        );
        assert_eq!(
            plane.local_intersect(Ray::new(Point3::from_vec(-vy), vy)),
            v
        );
    }

    #[test]
    fn local_normal_at() {
        let plane = Plane::default();
        let vy = Vector3::unit_y();
        assert_eq!(plane.local_normal_at(Point3::origin()), vy);
        assert_eq!(plane.local_normal_at(10. * Point3::new(1., 0., -1.)), vy);
        assert_eq!(plane.local_normal_at(Point3::new(-5., 0., 150.)), vy);
    }
}
