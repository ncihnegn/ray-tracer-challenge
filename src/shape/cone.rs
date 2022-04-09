use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, TraitShape},
};
use cgmath::{
    abs_diff_eq, abs_diff_ne, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix,
    Vector3,
};
use derive_more::Constructor;
use std::fmt::Debug;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Cone<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
    pub minimum: T,
    pub maximum: T,
    pub closed: bool,
}

impl<T: BaseFloat + Default> Default for Cone<T> {
    fn default() -> Cone<T> {
        Cone::<T> {
            transform: Matrix4::identity(),
            material: Material::default(),
            minimum: -T::infinity(),
            maximum: T::infinity(),
            closed: false,
        }
    }
}

fn check_cap<T: BaseFloat>(ray: Ray<T>, t: T, radius: T) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x.powi(2) + z.powi(2) <= radius.powi(2)
}

impl<T: BaseFloat> Cone<T> {
    fn intersect_caps(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let mut xs = Vec::new();
        if self.closed && abs_diff_ne!(ray.direction.y, T::zero()) {
            for m in [self.minimum, self.maximum] {
                let t = (m - ray.origin.y) / ray.direction.y;
                if check_cap(ray, t, m) {
                    xs.push(Intersection::new(t, Shape::Cone(self.clone())));
                }
            }
        }
        xs
    }
}

impl<T: BaseFloat + Debug> TraitShape<T> for Cone<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Option<Material<T>> {
        Some(self.material)
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let two = T::from(2).unwrap();
        let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);
        let b = two
            * (ray.origin.x * ray.direction.x - ray.origin.y * ray.direction.y
                + ray.origin.z * ray.direction.z);
        let c = ray.origin.x.powi(2) - ray.origin.y.powi(2) + ray.origin.z.powi(2);
        let mut xs = Vec::new();
        if abs_diff_eq!(a, T::zero()) && !abs_diff_eq!(b, T::zero()) {
            xs.push(Intersection::new(-c / (two * b), Shape::Cone(self.clone())));
        } else {
            let disc = b.powi(2) - T::from(4).unwrap() * a * c;
            if disc < T::zero() {
                return vec![];
            }
            let t0 = (-b - disc.sqrt()) / (two * a);
            let t1 = (-b + disc.sqrt()) / (two * a);
            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, Shape::Cone(self.clone())));
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, Shape::Cone(self.clone())));
            }
        }
        xs.append(&mut self.intersect_caps(ray));
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < T::one() && abs_diff_eq!(point.y, self.maximum) {
            Vector3::unit_y()
        } else if dist < T::one() && abs_diff_eq!(point.y, self.minimum) {
            -Vector3::unit_y()
        } else {
            Vector3::new(point.x, -point.y.signum() * dist.sqrt(), point.z)
        }
    }
}

mod tests {
    use crate::shape::cone;

    use super::*;
    use cgmath::{assert_relative_eq, Rad, Zero};
    use std::f32::consts::{PI, SQRT_2};

    #[test]
    fn local_intersect() {
        {
            let cone = Cone::default();
            let shape = Shape::Cone(cone.clone());
            assert_relative_eq!(
                cone.local_intersect(Ray::new(
                    Point3::new(0., 0., -1.),
                    Vector3::new(0., 1., 1.).normalize()
                ))
                .iter()
                .map(|i| i.t)
                .collect::<Vec<_>>()[..],
                vec![0.35355],
                max_relative = 0.00001
            );
            assert_eq!(
                cone.local_intersect(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
                vec![
                    Intersection::new(5., shape.clone()),
                    Intersection::new(5., shape)
                ]
            );
            assert_relative_eq!(
                cone.local_intersect(Ray::new(
                    Point3::new(0., 0., -5.),
                    Vector3::new(1., 1., 1.).normalize()
                ))
                .iter()
                .map(|i| i.t)
                .collect::<Vec<_>>()[..],
                vec![8.66025, 8.66025],
                max_relative = 0.00001
            );
            assert_relative_eq!(
                cone.local_intersect(Ray::new(
                    Point3::new(1., 1., -5.),
                    Vector3::new(-0.5, -1., 1.).normalize()
                ))
                .iter()
                .map(|i| i.t)
                .collect::<Vec<_>>()[..],
                [4.55006, 49.44994,],
                max_relative = 0.00001
            );
            assert_relative_eq!(
                cone.local_intersect(Ray::new(
                    Point3::new(0., 0., -1.),
                    Vector3::new(0., 1., 1.).normalize()
                ))
                .iter()
                .map(|i| i.t)
                .collect::<Vec<_>>()[..],
                vec![0.35355],
                max_relative = 0.00001
            );
        }
        {
            let mut cone = Cone::default();
            cone.minimum = -0.5;
            cone.maximum = 0.5;
            cone.closed = true;
            assert_eq!(
                cone.local_intersect(Ray::new(Point3::new(0., 0., -5.), -Vector3::unit_y())),
                vec![]
            );
            assert_eq!(
                cone.local_intersect(Ray::new(
                    Point3::new(0., 0., -0.25),
                    Vector3::new(0., 1., 1.).normalize()
                ))
                .len(),
                2
            );
            assert_eq!(
                cone.local_intersect(Ray::new(Point3::new(0., 0., -0.25), Vector3::unit_y()))
                    .len(),
                4
            );
        }
    }

    #[test]
    fn local_normal_at() {
        let cone = Cone::default();
        assert_eq!(cone.local_normal_at(Point3::origin()), Vector3::zero());
        assert_eq!(
            cone.local_normal_at(Point3::new(1., 1., 1.)),
            Vector3::new(1., -SQRT_2, 1.)
        );
        assert_eq!(
            cone.local_normal_at(Point3::new(-1., -1., 0.)),
            Vector3::new(-1., 1., 0.)
        );
    }
}