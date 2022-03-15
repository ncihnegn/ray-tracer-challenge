use crate::computation::Computation;
use crate::intersection::{hit, Intersection};
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use cgmath::{BaseFloat, EuclideanSpace, Matrix4, Point3};
use derive_more::Constructor;
use rgb::RGB;

#[derive(Constructor)]
pub struct World<T> {
    light: Option<Light<T>>,
    objects: Vec<Sphere<T>>,
}

impl<T: BaseFloat + Default> Default for World<T> {
    fn default() -> World<T> {
        let neg10 = T::from(-10).unwrap();
        let one = T::one();
        World::<T> {
            light: Some(Light::new(
                Point3::new(neg10, neg10, neg10),
                RGB::new(one, one, one),
            )),
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
    fn shade_hit(&self, comps: Computation<T>) -> Option<RGB<T>> {
        self.light.map(|light| {
            comps
                .object
                .material
                .lighting(light, comps.point, comps.eyev, comps.normalv)
        })
    }

    fn intersect(&self, ray: &Ray<T>) -> Vec<Intersection<T>> {
        self.objects
            .iter()
            .map(|s| ray.intersect(s))
            .into_iter()
            .flatten()
            .collect()
    }

    fn color_at(&self, ray: &Ray<T>) -> RGB<T> {
        if let Some(i) = hit(self.intersect(ray)) {
            self.shade_hit(i.precompute(ray)).unwrap()
        } else {
            RGB::default()
        }
    }
}

mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use cgmath::Vector3;

    #[test]
    fn shade_hit() {
        let mut w = World::<f32>::default();
        {
            let r = Ray::<f32>::new(Point3::new(0., 0., -5.), Vector3::unit_z());
            let i = Intersection::new(4., w.objects[0].clone());
            let comps = i.precompute(&r);
            assert_relative_eq!(
                w.shade_hit(comps).unwrap(),
                RGB::new(0.38066, 0.47583, 0.2855),
                max_relative = 0.0001
            );
        }
        {
            w.light = Some(Light::new(Point3::new(0., 0.25, 0.), RGB::new(1., 1., 1.)));
            let r = Ray::<f32>::new(Point3::origin(), Vector3::unit_z());
            let i = Intersection::new(0.5, w.objects[1].clone());
            let comps = i.precompute(&r);
            assert_relative_eq!(
                w.shade_hit(comps).unwrap(),
                RGB::new(0.90498, 0.90498, 0.90498),
                max_relative = 0.00001
            );
        }
    }

    #[test]
    fn color_at() {
        let mut w = World::<f32>::default();
        {
            let r = Ray::new(Point3::new(0., 0., -5.), Vector3::unit_y());
            assert_eq!(w.color_at(&r), RGB::default());
        }
        {
            let r = Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z());
            assert_relative_eq!(
                w.color_at(&r),
                RGB::new(0.38066, 0.47583, 0.2855),
                max_relative = 0.0001
            );
        }
        {
            w.objects[0].material.ambient = 1.;
            w.objects[1].material.ambient = 1.;
            let r = Ray::new(Point3::new(0., 0., 0.75), -Vector3::unit_z());
            assert_eq!(w.color_at(&r), w.objects[1].material.color);
        }
    }
}
