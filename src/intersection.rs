use crate::{
    computation::Computation,
    material::Material,
    ray::Ray,
    shape::{plane::Plane, reflect, sphere::Sphere, Shape, TraitShape},
};
use cgmath::{dot, BaseFloat, EuclideanSpace, Matrix4, Point3, SquareMatrix, Vector3};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Intersection<T> {
    pub t: T,
    pub object: Shape<T>,
}

impl<T: BaseFloat> Intersection<T> {
    pub fn precompute(&self, ray: Ray<T>, xs: &[Intersection<T>]) -> Option<Computation<T>> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        self.object.normal_at(point).map(|t_normalv| {
            let inside = dot(t_normalv, eyev) < T::zero();
            let normalv = if inside { -t_normalv } else { t_normalv };
            let reflectv = reflect(ray.direction, normalv);
            let mut n1 = None;
            let mut n2 = None;
            let mut containers = Vec::<Shape<T>>::new();
            for i in xs {
                if self == i {
                    n1 = containers
                        .last()
                        .map(|i| i.material().unwrap().refractive_index);
                }
                if let Some(index) = containers.iter().position(|x| *x == i.object) {
                    containers.remove(index);
                } else {
                    containers.push(i.object.clone());
                }
                if self == i {
                    n2 = containers
                        .last()
                        .map(|i| i.material().unwrap().refractive_index);
                    break;
                }
            }
            Computation::new(
                self.t,
                self.object.clone(),
                point,
                eyev,
                normalv,
                inside,
                reflectv,
                n1.unwrap_or_else(T::one),
                n2.unwrap_or_else(T::one),
            )
        })
    }
}

pub fn hit<T: BaseFloat>(v: &[Intersection<T>]) -> Option<Intersection<T>> {
    v.iter()
        .filter(|i| i.t >= T::from(f32::EPSILON).unwrap()) // -0.0 >= T::zero()
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .cloned()
}

mod tests {
    use super::*;
    use std::f32::{consts::FRAC_1_SQRT_2, EPSILON};

    #[test]
    fn hit() {
        let sphere = Shape::Sphere(Sphere::default());
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

    #[test]
    fn precompute() {
        {
            let vz = Vector3::unit_z();
            let r = Ray::new(Point3::from_vec(-5. * vz), vz);
            let shape = Shape::Sphere(Sphere::new(
                Matrix4::from_translation(vz),
                Material::default(),
            ));
            let i = Intersection::new(5., shape);
            let xs = vec![i.clone()];
            let comps = i.precompute(r, &xs).unwrap();
            assert!(comps.over_point().z < EPSILON / 2.);
            assert!(comps.point.z > comps.over_point().z);
            assert!(comps.under_point().z > EPSILON / 2.);
            assert!(comps.point.z < comps.under_point().z);
        }
        {
            let object = Shape::Sphere(Sphere::default());
            let vz = Vector3::unit_z();
            let point = Point3::from_vec(vz);
            let i = Intersection::new(1., object.clone());
            let xs = vec![i.clone()];
            assert_eq!(
                i.precompute(Ray::new(Point3::origin(), vz), &xs).unwrap(),
                Computation::new(1., object, point, -vz, -vz, true, -vz, 1., 1.)
            );
        }
        {
            let shape = Plane::default();
            let r = Ray::new(
                Point3::new(0., 1., -1.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let i = Intersection::new(2.0_f32.sqrt(), Shape::Plane(shape));
            let xs = vec![i.clone()];
            let comps = i.precompute(r, &xs).unwrap();
            assert_eq!(
                comps.reflectv,
                Vector3::new(0., FRAC_1_SQRT_2, FRAC_1_SQRT_2)
            );
        }
        {
            let vz = Vector3::unit_z();
            let mut material = Material::default();
            material.transparency = 1.;
            material.refractive_index = 1.5;
            let a = Shape::Sphere(Sphere::new(Matrix4::from_scale(2.), material));
            material.refractive_index = 2.;
            let b = Shape::Sphere(Sphere::new(Matrix4::from_translation(vz * -0.25), material));
            material.refractive_index = 2.5;
            let c = Shape::Sphere(Sphere::new(Matrix4::from_translation(vz * 0.25), material));
            let r = Ray::new(Point3::from_vec(vz * -4.), vz);
            let xs = vec![
                Intersection::new(2., a.clone()),
                Intersection::new(2.75, b.clone()),
                Intersection::new(3.25, c.clone()),
                Intersection::new(4.75, b),
                Intersection::new(5.25, c),
                Intersection::new(6., a),
            ];
            let n1s = vec![1., 1.5, 2., 2.5, 2.5, 1.5];
            let n2s = vec![1.5, 2., 2.5, 2.5, 1.5, 1.];
            for index in 0..=5 {
                let i = xs[index].clone();
                let comps = i.precompute(r, &xs).unwrap();
                assert_eq!(comps.n1, n1s[index]);
                assert_eq!(comps.n2, n2s[index]);
            }
        }
    }
}
