use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, TraitShape},
};
use cgmath::{
    abs_diff_eq, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3,
};
use derive_more::Constructor;

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
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

impl<T: BaseFloat> TraitShape<T> for Plane<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Material<T> {
        self.material
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if abs_diff_eq!(ray.direction.y, T::zero()) {
            Vec::new()
        } else {
            vec![Intersection::new(
                -ray.origin.y / ray.direction.y,
                Shape::Plane(*self),
            )]
        }
    }

    fn local_normal_at(&self, _point: Point3<T>) -> Vector3<T> {
        Vector3::unit_y()
    }
}

mod tests {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn local_intersect() {
        let plane = Plane::default();
        let shape = Shape::Plane(plane);
        let vy = Vector3::unit_y();
        let v = vec![Intersection::new(1., shape)];
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
