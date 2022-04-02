use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, TraitShape},
};
use cgmath::{BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
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
        if ray.direction.y.abs() < T::epsilon() {
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
