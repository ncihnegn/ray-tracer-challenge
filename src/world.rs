use crate::computation::Computation;
use crate::intersection::{hit, Intersection};
use crate::light::Light;
use crate::material::Material;
use crate::pattern::Pattern;
use crate::ray::Ray;
use crate::shape::{Plane, Shape, Sphere, TraitShape};
use cgmath::{BaseFloat, InnerSpace, Matrix4, Point3};
use derive_more::Constructor;
use rgb::RGB;

#[derive(Constructor)]
pub struct World<T> {
    light: Light<T>,
    objects: Vec<Shape<T>>,
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
                Shape::Sphere(Sphere::new(
                    Matrix4::from_scale(one),
                    Material::new(
                        Pattern::Solid(RGB::new(
                            T::from(0.8).unwrap(),
                            T::one(),
                            T::from(0.6).unwrap(),
                        )),
                        T::from(0.1).unwrap(),
                        T::from(0.7).unwrap(),
                        T::from(0.2).unwrap(),
                        T::from(200).unwrap(),
                        T::zero(),
                    ),
                )),
                Shape::Sphere(Sphere::new(
                    Matrix4::from_scale(T::from(0.5).unwrap()),
                    Material::default(),
                )),
            ],
        }
    }
}

impl<T: BaseFloat + Default> World<T> {
    fn shade_hit(&self, comps: Computation<T>, remaining: u8) -> RGB<T> {
        let shadowed = self.is_shadowed(comps.over_point());
        let surface = comps.object.material().lighting(
            self.light,
            comps.point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps, remaining);
        surface + reflected
    }

    fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        self.objects
            .iter()
            .map(|&s| s.intersect(ray))
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn color_at(&self, ray: Ray<T>, remaining: u8) -> RGB<T> {
        if let Some(i) = hit(&self.intersect(ray)) {
            self.shade_hit(i.precompute(ray), remaining)
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

    fn reflected_color(&self, comps: Computation<T>, remaining: u8) -> RGB<T> {
        let r = comps.object.material().reflective;
        if remaining == 0 || r == T::zero() {
            // Terminate the calling loop
            RGB::default()
        } else {
            let reflect_ray = Ray::new(comps.over_point(), comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * r
        }
    }
}

mod tests {
    use std::f32::consts::FRAC_1_SQRT_2;

    use super::*;
    use approx::assert_relative_eq;
    use cgmath::{EuclideanSpace, Vector3};

    #[test]
    fn shade_hit() {
        {
            let w = World::default();
            assert_relative_eq!(
                w.shade_hit(
                    Intersection::new(4., w.objects[0])
                        .precompute(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
                    1
                ),
                RGB::new(0.38066, 0.47583, 0.2855),
                max_relative = 0.0001
            );
        }
        {
            let mut w = World::default();
            w.light = Light::new(Point3::new(0., 0.25, 0.), RGB::new(1., 1., 1.));
            assert_relative_eq!(
                w.shade_hit(
                    Intersection::new(0.5, w.objects[1])
                        .precompute(Ray::new(Point3::origin(), Vector3::unit_z())),
                    1
                ),
                RGB::new(0.90498, 0.90498, 0.90498),
                max_relative = 0.00001
            );
        }
        {
            let mut w = World::default();
            let mut plane = Plane::default();
            plane.transform = Matrix4::from_translation(-Vector3::unit_y());
            plane.material.reflective = 0.5;
            let shape = Shape::Plane(plane);
            w.objects.push(shape);
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let i = Intersection::new(2.0_f32.sqrt(), shape);
            let comps = i.precompute(r);
            assert_relative_eq!(
                w.shade_hit(comps, 1),
                RGB::new(0.87677, 0.92436, 0.82918),
                max_relative = 0.0001
            );
        }
    }

    #[test]
    fn color_at() {
        let mut w = World::default();
        {
            assert_eq!(
                w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_y()), 1),
                RGB::default()
            );
            assert_relative_eq!(
                w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()), 1),
                RGB::new(0.38066, 0.47583, 0.2855),
                max_relative = 0.0001
            );
        }
        {
            w.objects[0].as_sphere_mut().unwrap().material.ambient = 1.;
            w.objects[1].as_sphere_mut().unwrap().material.ambient = 1.;
            assert_eq!(
                w.color_at(Ray::new(Point3::new(0., 0., 0.75), -Vector3::unit_z()), 1),
                w.objects[1].material().pattern.at(Point3::origin())
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

    #[test]
    fn reflected_color() {
        let mut w = World::default();
        {
            let r = Ray::new(Point3::origin(), Vector3::unit_z());
            let mut shape = w.objects[1];
            shape.as_sphere_mut().unwrap().material.ambient = 1.;
            let i = Intersection::new(1., shape);
            let comps = i.precompute(r);
            assert_eq!(w.reflected_color(comps, 1), RGB::default());
        }
        {
            let mut plane = Plane::default();
            plane.transform = Matrix4::from_translation(-Vector3::unit_y());
            plane.material.reflective = 0.5;
            let shape = Shape::Plane(plane);
            w.objects.push(shape);
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let i = Intersection::new(2.0_f32.sqrt(), shape);
            let comps = i.precompute(r);
            assert_eq!(w.reflected_color(comps, 0), RGB::default());
            assert_relative_eq!(
                w.reflected_color(comps, 1),
                RGB::new(0.19032, 0.2379, 0.14274),
                max_relative = 0.0001
            );
        }
    }

    #[test]
    fn infinite_recursion() {
        let mut w = World::default();
        w.light = Light::new(Point3::origin(), RGB::new(1., 1., 1.));
        {
            let mut plane = Plane::default();
            plane.transform = Matrix4::from_translation(-Vector3::unit_y());
            plane.material.reflective = 1.;
            w.objects[0] = Shape::Plane(plane);
        }
        {
            let mut plane = Plane::default();
            plane.transform = Matrix4::from_translation(Vector3::unit_y());
            plane.material.reflective = 1.;
            w.objects[1] = Shape::Plane(plane);
        }
        let r = Ray::new(Point3::origin(), Vector3::unit_y());
        let c = w.color_at(r, 1);
    }
}
