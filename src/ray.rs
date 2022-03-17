use crate::impl_approx;
use crate::intersection::Intersection;
use crate::sphere::Sphere;
use cgmath::{
    AbsDiffEq, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, RelativeEq, SquareMatrix,
    UlpsEq, Vector3,
};
use derive_more::Constructor;

#[derive(Constructor, Debug, PartialEq)]
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

    // Find intersection with the unit sphere.
    fn intersect_unit(&self) -> Vec<T> {
        let sphere_to_ray = self.origin.to_vec();
        let a = self.direction.dot(self.direction);
        let two = T::from(2).unwrap();
        let b = self.direction.dot(sphere_to_ray) * two;
        let c = sphere_to_ray.dot(sphere_to_ray) - T::one();
        let discriminant = b * b - T::from(4).unwrap() * a * c;
        match discriminant {
            d if d > T::zero() => vec![(-b - d.sqrt()) / (two * a), (-b + d.sqrt()) / (two * a)],
            d if d == T::zero() => vec![-b / (two * a)],
            _ => vec![],
        }
    }

    fn transform(&self, transform: Matrix4<T>) -> Ray<T> {
        Ray::<T> {
            origin: Point3::from_homogeneous(transform * self.origin.to_homogeneous()),
            direction: (transform * self.direction.extend(T::zero())).truncate(),
        }
    }

    pub fn intersect(&self, object: &Sphere<T>) -> Vec<Intersection<T>> {
        if let Some(t) = object.transform.invert() {
            self.transform(t)
                .intersect_unit()
                .iter()
                .map(|&t| Intersection::new(t, object.clone()))
                .collect()
        } else {
            Vec::new()
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
    fn intersect_unit() {
        // A ray intersecs the sphere at two points
        assert_eq!(
            Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()).intersect_unit(),
            vec![4., 6.]
        );
        // A ray intersecs the sphere at a tangent.
        assert_eq!(
            Ray::new(Point3::new(0., 1., -5.), Vector3::unit_z()).intersect_unit(),
            vec![5.]
        );
        // A ray misses the sphere.
        assert!(Ray::new(Point3::new(0., 2., -5.), Vector3::unit_z())
            .intersect_unit()
            .is_empty());

        // A ray originates inside the sphere.
        assert_eq!(
            Ray::<f32>::new(Point3::origin(), Vector3::unit_z(),).intersect_unit(),
            vec![-1., 1.]
        );
        // A ray is in front of the sphere.
        assert_eq!(
            Ray::new(Point3::new(0., 0., 5.), Vector3::unit_z()).intersect_unit(),
            vec![-6., -4.]
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

    #[test]
    fn intersect() {
        {
            let object = Sphere::new(Matrix4::from_scale(2.), Material::default());
            assert_eq!(
                Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z(),).intersect(&object),
                vec![
                    Intersection::new(3., object.clone()),
                    Intersection::new(7., object)
                ]
            );
        }
        {
            let translation = Matrix4::from_translation(Vector3::new(5., 0., 0.));
            let object = Sphere::new(translation, Material::default());
            assert_eq!(
                Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z(),).intersect(&object),
                vec![]
            );
        }
    }
}
