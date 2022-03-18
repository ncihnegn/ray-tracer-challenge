use crate::computation::Computation;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::{normal_at, Sphere};
use cgmath::{dot, BaseFloat, EuclideanSpace, Matrix4, Point3, Vector3};
use derive_more::Constructor;

#[derive(Constructor, Debug, Clone, PartialEq)]
pub struct Intersection<T> {
    pub t: T,
    pub object: Sphere<T>,
}

impl<T: BaseFloat> Intersection<T> {
    pub fn precompute(&self, ray: &Ray<T>) -> Computation<T> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let t_normalv = normal_at(self.object.transform, point).unwrap();
        let inside = dot(t_normalv, eyev) < T::zero();
        let normalv = if inside { -t_normalv } else { t_normalv };
        Computation::new(self.object.clone(), self.t, point, eyev, normalv, inside)
    }
}

pub fn hit<T: BaseFloat>(v: &Vec<Intersection<T>>) -> Option<Intersection<T>> {
    v.iter()
        .filter(|i| i.t >= T::zero())
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .cloned()
}

mod tests {
    use super::*;
    use num_traits::Float;

    #[test]
    fn hit() {
        let sphere = Sphere::new(Matrix4::from_scale(1.), Material::default());
        {
            // All have positive t
            let i1 = Intersection::new(1., sphere.clone());
            assert_eq!(
                super::hit(&vec![i1.clone(), Intersection::new(2., sphere.clone())]),
                Some(i1.clone())
            );
            // Some have negative t
            assert_eq!(
                super::hit(&vec![Intersection::new(-1., sphere.clone()), i1.clone()]),
                Some(i1)
            );
        }
        // All have negative t
        assert_eq!(
            super::hit(&vec![
                Intersection::new(-2., sphere.clone()),
                Intersection::new(-1., sphere)
            ]),
            None
        );
    }

    fn precompute() {
        {
            let object = Sphere::new(Matrix4::from_scale(1.), Material::default());
            let point = Point3::<f32>::new(0., 0., -1.);
            let v = -Vector3::<f32>::unit_z();
            assert_eq!(
                Intersection::new(1., object.clone())
                    .precompute(&Ray::new(Point3::origin(), Vector3::unit_z())),
                Computation::new(object, 1., point, v, v, true)
            );
        }
        {
            let r = Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z());
            let shape = Sphere::new(
                Matrix4::from_translation(Vector3::unit_z()),
                Material::default(),
            );
            let comps = Intersection::new(5., shape.clone())
                .precompute(&Ray::new(Point3::origin(), Vector3::unit_z()));
            assert!(comps.over_point().z < -f32::EPSILON / 2.);
            assert!(comps.point.z > comps.over_point().z);
        }
    }
}
