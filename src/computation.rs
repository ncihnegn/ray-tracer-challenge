use crate::shape::Shape;
use cgmath::{BaseFloat, InnerSpace, Point3, Vector3};
use derive_more::Constructor;

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Computation<T> {
    pub t: T,
    pub object: Shape<T>,
    pub point: Point3<T>,
    pub eyev: Vector3<T>,
    pub normalv: Vector3<T>,
    pub inside: bool,
    pub reflectv: Vector3<T>,
    pub n1: T,
    pub n2: T,
}

impl<T: BaseFloat> Computation<T> {
    pub fn over_point(&self) -> Point3<T> {
        self.point + self.normalv * T::epsilon()
    }

    pub fn under_point(&self) -> Point3<T> {
        self.point - self.normalv * T::epsilon()
    }

    pub fn n_ratio(&self) -> T {
        self.n1 / self.n2
    }

    pub fn cos_i(&self) -> T {
        self.eyev.dot(self.normalv)
    }

    pub fn sin2_t(&self) -> T {
        self.n_ratio() * self.n_ratio() * (T::one() - self.cos_i() * self.cos_i())
    }

    pub fn cos_t(&self) -> T {
        (T::one() - self.sin2_t()).sqrt()
    }

    pub fn schlick(&self) -> T {
        let one = T::one();
        if self.n1 > self.n2 {
            if self.sin2_t() > T::one() {
                return one;
            }
        }
        let cos = if self.n1 > self.n2 {
            self.cos_t()
        } else {
            self.cos_i()
        };
        let r0_t = (self.n1 - self.n2) / (self.n1 + self.n2);
        let r0 = r0_t * r0_t;
        r0 + (one - r0) * (one - cos).powf(T::from(5).unwrap())
    }
}

mod tests {
    use cgmath::{assert_relative_eq, EuclideanSpace};
    use std::f32::consts::FRAC_1_SQRT_2;

    use crate::{
        intersection::Intersection,
        ray::Ray,
        shape::{Shape, Sphere},
    };

    use super::*;

    #[test]
    fn schlick() {
        let mut sphere = Sphere::default();
        sphere.material.transparency = 1.0;
        sphere.material.refractive_index = 1.5;
        let shape = Shape::Sphere(sphere);
        {
            let r = Ray::new(Point3::new(0., 0., FRAC_1_SQRT_2), Vector3::unit_y());
            let xs = vec![
                Intersection::new(-FRAC_1_SQRT_2, shape),
                Intersection::new(FRAC_1_SQRT_2, shape),
            ];
            let comps = xs[1].precompute(r, &xs).unwrap();
            assert_eq!(comps.schlick(), 1.0);
        }
        {
            let r = Ray::new(Point3::origin(), Vector3::unit_y());
            let xs = vec![Intersection::new(-1., shape), Intersection::new(1., shape)];
            let comps = xs[1].precompute(r, &xs).unwrap();
            assert_relative_eq!(comps.schlick(), 0.04);
        }
        {
            let r = Ray::new(Point3::new(0., 0.99, -2.), Vector3::unit_z());
            let xs = vec![Intersection::new(1.8589, shape)];
            let comps = xs[0].precompute(r, &xs).unwrap();
            assert_relative_eq!(comps.schlick(), 0.48873, max_relative = 0.00001);
        }
    }
}
