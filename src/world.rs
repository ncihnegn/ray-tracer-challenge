use crate::{
    computation::Computation,
    intersection::{hit, Intersection},
    light::Light,
    material::Material,
    pattern::Pattern,
    ray::Ray,
    shape::{sphere::Sphere, Shape},
};
use cgmath::{BaseFloat, InnerSpace, Matrix4, Point3, SquareMatrix};
use rgb::RGB;

const RECURSION_LIMIT: u8 = 5;

#[derive(derive_more::Constructor, Debug)]
pub struct World<T> {
    pub light: Light<T>,
    pub objects: Vec<Shape<T>>,
    recursion: u8,
}

impl<T: BaseFloat + Default> Default for World<T> {
    fn default() -> World<T> {
        let neg10 = T::from(-10).unwrap();
        let one = T::one();
        World::<T> {
            light: Light::new(Point3::new(neg10, -neg10, neg10), RGB::new(one, one, one)),
            objects: vec![
                Shape::Sphere(Sphere::new(
                    Matrix4::identity(),
                    Material::new(
                        Pattern::Solid(RGB::new(T::from(0.8).unwrap(), one, T::from(0.6).unwrap())),
                        T::from(0.1).unwrap(),
                        T::from(0.7).unwrap(),
                        T::from(0.2).unwrap(),
                        T::from(200).unwrap(),
                        T::zero(),
                        T::zero(),
                        one,
                    ),
                    None,
                )),
                Shape::Sphere(Sphere::new(
                    Matrix4::from_scale(T::from(0.5).unwrap()),
                    Material::default(),
                    None,
                )),
            ],
            recursion: RECURSION_LIMIT,
        }
    }
}

impl<T: BaseFloat + Default> World<T> {
    fn shade_hit(&mut self, comps: &Computation<T>) -> RGB<T> {
        let shadowed = self.is_shadowed(comps.over_point());
        let material = comps.object.material().unwrap();
        let surface = material.lighting(
            self.light,
            comps.over_point(),
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps);
        let refracted = self.refracted_color(comps);
        if material.reflective > T::zero() && material.transparency > T::zero() {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (T::one() - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let mut xs = self
            .objects
            .iter()
            .map(|s| s.intersect(ray))
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        xs.sort_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap_or(std::cmp::Ordering::Less));
        xs
    }

    pub fn color_at(&mut self, ray: Ray<T>) -> RGB<T> {
        let xs = self.intersect(ray);
        if let Some(i) = hit(&xs) {
            if let Some(comps) = i.precompute(ray, &xs) {
                self.shade_hit(&comps)
            } else {
                RGB::default()
            }
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

    fn reflected_color(&mut self, comps: &Computation<T>) -> RGB<T> {
        let r = comps.object.material().unwrap().reflective;
        if self.recursion == 0 || r == T::zero() {
            // Terminate the calling loop
            self.recursion = RECURSION_LIMIT;
            RGB::default()
        } else {
            let reflect_ray = Ray::new(comps.over_point(), comps.reflectv);
            self.recursion -= 1;
            let color = self.color_at(reflect_ray);
            color * r
        }
    }

    fn refracted_color(&mut self, comps: &Computation<T>) -> RGB<T> {
        let material = comps.object.material().unwrap();
        if self.recursion == 0 || material.transparency == T::zero() {
            self.recursion = RECURSION_LIMIT;
            RGB::default()
        } else {
            let one = T::one();
            if comps.sin2_t() > one {
                RGB::default()
            } else {
                let direction = comps.normalv * (comps.n_ratio() * comps.cos_i() - comps.cos_t())
                    - comps.eyev * comps.n_ratio();
                let refracted_ray = Ray::new(comps.under_point(), direction);
                self.recursion -= 1;
                self.color_at(refracted_ray) * material.transparency
            }
        }
    }
}

mod tests {
    use super::*;
    use crate::{pattern::test::Test, shape::plane::Plane};
    use approx::assert_relative_eq;
    use cgmath::{EuclideanSpace, Vector3};
    use std::f32::consts::FRAC_1_SQRT_2;

    #[test]
    fn shade_hit() {
        {
            let mut w = World::default();
            let xs = vec![Intersection::new(4., w.objects[0].clone(), None)];
            assert_relative_eq!(
                w.shade_hit(
                    &xs[0]
                        .precompute(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()), &xs)
                        .unwrap(),
                ),
                RGB::new(0.38066, 0.47583, 0.2855),
                max_relative = 0.0001
            );
        }
        {
            let mut w = World::default();
            w.light = Light::new(Point3::new(0., 0.25, 0.), RGB::new(1., 1., 1.));
            let xs = vec![Intersection::new(0.5, w.objects[1].clone(), None)];
            assert_relative_eq!(
                w.shade_hit(
                    &xs[0]
                        .precompute(Ray::new(Point3::origin(), Vector3::unit_z()), &xs)
                        .unwrap(),
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
            w.objects.push(shape.clone());
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let xs = vec![Intersection::new(2.0_f32.sqrt(), shape, None)];
            let comps = xs[0].precompute(r, &xs).unwrap();
            assert_relative_eq!(
                w.shade_hit(&comps),
                RGB::new(0.87677, 0.92436, 0.82918),
                max_relative = 0.0001
            );
        }
        {
            let mut w = World::default();
            let mut floor = Plane::default();
            floor.transform = Matrix4::from_translation(-Vector3::unit_y());
            floor.material.transparency = 0.5;
            floor.material.refractive_index = 1.5;
            let mut ball = Sphere::default();
            ball.material.pattern = Pattern::Solid(RGB::new(1., 0., 0.));
            ball.material.ambient = 0.5;
            ball.transform = Matrix4::from_translation(Vector3::new(0., -3.5, -0.5));
            let shape = Shape::Plane(floor);
            w.objects.push(shape.clone());
            w.objects.push(Shape::Sphere(ball));
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let xs = vec![Intersection::new(2.0_f32.sqrt(), shape, None)];
            let comps = xs[0].precompute(r, &xs).unwrap();
            assert_relative_eq!(
                w.shade_hit(&comps),
                RGB::new(0.93642, 0.68642, 0.68642),
                max_relative = 0.00001
            );
        }
        {
            let mut w = World::default();
            let mut floor = Plane::default();
            floor.transform = Matrix4::from_translation(-Vector3::unit_y());
            floor.material.reflective = 0.5;
            floor.material.transparency = 0.5;
            floor.material.refractive_index = 1.5;
            let mut ball = Sphere::default();
            ball.material.pattern = Pattern::Solid(RGB::new(1., 0., 0.));
            ball.material.ambient = 0.5;
            ball.transform = Matrix4::from_translation(Vector3::new(0., -3.5, -0.5));
            let shape = Shape::Plane(floor);
            w.objects.push(shape.clone());
            w.objects.push(Shape::Sphere(ball));
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let xs = vec![Intersection::new(2.0_f32.sqrt(), shape, None)];
            let comps = xs[0].precompute(r, &xs).unwrap();
            assert_relative_eq!(
                w.shade_hit(&comps),
                RGB::new(0.93391, 0.69643, 0.69243),
                max_relative = 0.00001
            );
        }
    }

    #[test]
    fn color_at() {
        let mut w = World::default();
        assert_eq!(
            w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_y())),
            RGB::default()
        );
        assert_relative_eq!(
            w.color_at(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
            RGB::new(0.38066, 0.47583, 0.2855),
            max_relative = 0.0001
        );
        w.objects[0].as_sphere_mut().unwrap().material.ambient = 1.;
        w.objects[1].as_sphere_mut().unwrap().material.ambient = 1.;
        assert_eq!(
            w.color_at(Ray::new(Point3::new(0., 0., 0.75), -Vector3::unit_z())),
            w.objects[1]
                .material()
                .unwrap()
                .pattern
                .at(Point3::origin())
        );
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
        let xs = Vec::new();
        {
            let r = Ray::new(Point3::origin(), Vector3::unit_z());
            w.objects[1].as_sphere_mut().unwrap().material.ambient = 1.;
            let i = Intersection::new(1., w.objects[1].clone(), None);
            let comps = i.precompute(r, &xs).unwrap();
            assert_eq!(w.reflected_color(&comps), RGB::default());
        }
        {
            let mut plane = Plane::default();
            plane.transform = Matrix4::from_translation(-Vector3::unit_y());
            plane.material.reflective = 0.5;
            let shape = Shape::Plane(plane);
            w.objects.push(shape.clone());
            let r = Ray::new(
                Point3::from_vec(Vector3::unit_z() * -3.),
                Vector3::new(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            );
            let i = Intersection::new(2.0_f32.sqrt(), shape, None);
            let comps = i.precompute(r, &xs).unwrap();
            w.recursion = 0;
            assert_eq!(w.reflected_color(&comps), RGB::default());
            w.recursion = 1;
            assert_relative_eq!(
                w.reflected_color(&comps),
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
        let _ = w.color_at(r);
    }

    #[test]
    fn refracted_color() {
        let vz = Vector3::unit_z();
        {
            let mut w = World::default();
            let shape = w.objects[0].clone();
            let xs = vec![
                Intersection::new(4., shape.clone(), None),
                Intersection::new(6., shape, None),
            ];
            let comps = xs[0]
                .precompute(Ray::new(Point3::from_vec(vz * -5.), vz), &xs)
                .unwrap();
            assert_eq!(w.refracted_color(&comps), RGB::default());
        }
        {
            let mut w = World::default();
            w.objects[0].as_sphere_mut().unwrap().material.transparency = 1.0;
            w.objects[0]
                .as_sphere_mut()
                .unwrap()
                .material
                .refractive_index = 1.5;
            let shape = w.objects[0].clone();
            let xs = vec![
                Intersection::new(-FRAC_1_SQRT_2, shape.clone(), None),
                Intersection::new(FRAC_1_SQRT_2, shape, None),
            ];
            let comps = xs[1]
                .precompute(
                    Ray::new(Point3::from_vec(vz * FRAC_1_SQRT_2), Vector3::unit_y()),
                    &xs,
                )
                .unwrap();
            assert_eq!(w.refracted_color(&comps), RGB::default());
        }
        {
            let mut w = World::default();
            w.objects[0].as_sphere_mut().unwrap().material.ambient = 1.;
            w.objects[0].as_sphere_mut().unwrap().material.pattern =
                Pattern::Test(Test::new(Matrix4::identity()));
            w.objects[1].as_sphere_mut().unwrap().material.transparency = 1.;
            w.objects[1]
                .as_sphere_mut()
                .unwrap()
                .material
                .refractive_index = 1.5;
            let xs = vec![
                Intersection::new(-0.9899, w.objects[0].clone(), None),
                Intersection::new(-0.4899, w.objects[1].clone(), None),
                Intersection::new(0.4899, w.objects[1].clone(), None),
                Intersection::new(0.9899, w.objects[0].clone(), None),
            ];
            let comps = xs[2]
                .precompute(Ray::new(Point3::from_vec(vz * 0.1), Vector3::unit_y()), &xs)
                .unwrap();
            assert_relative_eq!(
                w.refracted_color(&comps),
                RGB::new(0., 0.99888, 0.04725),
                max_relative = 0.001
            );
        }
    }
}
