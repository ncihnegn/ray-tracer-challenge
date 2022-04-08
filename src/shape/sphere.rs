use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, TraitShape},
};
use cgmath::{BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Sphere<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T: BaseFloat + Default> Default for Sphere<T> {
    fn default() -> Sphere<T> {
        Sphere::<T> {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

impl<T: BaseFloat> TraitShape<T> for Sphere<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Option<Material<T>> {
        Some(self.material)
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let sphere_to_ray = ray.origin.to_vec();
        let a = ray.direction.dot(ray.direction);
        let two = T::from(2).unwrap();
        let b = ray.direction.dot(sphere_to_ray) * two;
        let c = sphere_to_ray.dot(sphere_to_ray) - T::one();
        let discriminant = b.powi(2) - T::from(4).unwrap() * a * c;
        match discriminant {
            d if d > T::zero() => vec![
                Intersection::new((-b - d.sqrt()) / (two * a), Shape::Sphere(self.clone())),
                Intersection::new((-b + d.sqrt()) / (two * a), Shape::Sphere(self.clone())),
            ],
            d if d == T::zero() => vec![Intersection::new(
                -b / (two * a),
                Shape::Sphere(self.clone()),
            )],
            _ => vec![],
        }
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        point.to_vec()
    }
}

mod tests {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn local_normal_at() {
        {
            let sphere = Sphere::default();
            {
                let v = Vector3::unit_x();
                assert_eq!(sphere.local_normal_at(Point3::from_vec(v)), v);
            }
            {
                let v = Vector3::unit_y();
                assert_eq!(sphere.local_normal_at(Point3::from_vec(v)), v);
            }
            {
                let v = Vector3::unit_z();
                assert_eq!(sphere.local_normal_at(Point3::from_vec(v)), v);
            }
            {
                let v = 3.0_f32.sqrt().recip() * Vector3::new(1., 1., 1.);
                assert_relative_eq!(sphere.local_normal_at(Point3::from_vec(v)), v);
            }
        }
    }
}
