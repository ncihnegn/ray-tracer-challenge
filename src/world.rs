use crate::computation::Computation;
use crate::intersection::{hit, Intersection};
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Sphere;
use cgmath::{BaseFloat, InnerSpace, Matrix4, Point3};
use derive_more::Constructor;
use rgb::RGB;

#[derive(Constructor)]
pub struct World<T> {
    light: Light<T>,
    objects: Vec<Sphere<T>>,
}

impl<T: BaseFloat + Default> Default for World<T> {
    fn default() -> World<T> {
        let neg10 = T::from(-10).unwrap();
        let one = T::one();
        World::<T> {
            light: Light::new(
                Point3::new(neg10, T::from(10).unwrap(), neg10),
                RGB::new(one, one, one),
            ),
            objects: vec![
                Sphere::new(
                    Matrix4::from_scale(one),
                    Material::new(
                        RGB::new(T::from(0.8).unwrap(), T::one(), T::from(0.6).unwrap()),
                        T::from(0.1).unwrap(),
                        T::from(0.7).unwrap(),
                        T::from(0.2).unwrap(),
                        T::from(200).unwrap(),
                    ),
                ),
                Sphere::new(
                    Matrix4::from_scale(T::from(0.5).unwrap()),
                    Material::default(),
                ),
            ],
        }
    }
}

impl<T: BaseFloat + Default> World<T> {
    fn shade_hit(&self, comps: Computation<T>) -> RGB<T> {
        comps.object.material.lighting(
            self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(comps.over_point()),
        )
    }

    fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        self.objects
            .iter()
            .map(|&s| ray.intersect(s))
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn color_at(&self, ray: Ray<T>) -> RGB<T> {
        if let Some(i) = hit(&self.intersect(ray)) {
            self.shade_hit(i.precompute(ray))
        } else {
            RGB::default()
        }
    }

    fn is_shadowed(&self, point: Point3<T>) -> bool {
        let v = self.light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let intersections = self.intersect(Ray::new(point, direction));
        let h = hit(&intersections);
        h.is_some() && h.unwrap().t < distance
    }
}

mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use cgmath::{EuclideanSpace, Vector3};

    #[test]
    fn shade_hit() {
        let mut w = World::<f32>::default();
        assert_relative_eq!(
            w.shade_hit(
                Intersection::new(4., w.objects[0])
                    .precompute(Ray::<f32>::new(Point3::new(0., 0., -5.), Vector3::unit_z()))
            ),
            RGB::new(0.38066, 0.47583, 0.2855),
            max_relative = 0.0001
        );
        {
            w.light = Light::new(Point3::new(0., 0.25, 0.), RGB::new(1., 1., 1.));
            assert_relative_eq!(
                w.shade_hit(
                    Intersection::new(0.5, w.objects[1])
                        .precompute(Ray::<f32>::new(Point3::origin(), Vector3::unit_z()))
                ),
                RGB::new(0.90498, 0.90498, 0.90498),
                max_relative = 0.00001
            );
        }
    }

    #[test]
    fn color_at() {
        let mut w = World::<f32>::default();
        assert_eq!(
            w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_y())),
            RGB::default()
        );
        assert_relative_eq!(
            w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
            RGB::new(0.38066, 0.47583, 0.2855),
            max_relative = 0.0001
        );
        {
            w.objects[0].material.ambient = 1.;
            w.objects[1].material.ambient = 1.;
            assert_eq!(
                w.color_at(Ray::new(Point3::new(0., 0., 0.75), -Vector3::unit_z())),
                w.objects[1].material.color
            );
        }
    }

    #[test]
    fn is_shadowed() {
        let w = World::default();
        assert!(!w.is_shadowed(Point3::new(0., 10., 0.)));
        assert!(w.is_shadowed(Point3::new(10., -10., 10.)));
        assert!(!w.is_shadowed(Point3::new(-20., 20., -20.)));
        assert!(!w.is_shadowed(Point3::new(-2., 2., -2.)));
    }
}
