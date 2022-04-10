use crate::{intersection::Intersection, material::Material, ray::Ray, shape::Shape};
use cgmath::{
    abs_diff_eq, abs_diff_ne, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix,
    Vector3,
};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Cylinder<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
    pub minimum: T,
    pub maximum: T,
    pub closed: bool,
}

impl<T: BaseFloat + Default> Default for Cylinder<T> {
    fn default() -> Cylinder<T> {
        Cylinder::<T> {
            transform: Matrix4::identity(),
            material: Material::default(),
            minimum: -T::infinity(),
            maximum: T::infinity(),
            closed: false,
        }
    }
}

fn check_cap<T: BaseFloat>(ray: Ray<T>, t: T) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    x.powi(2) + z.powi(2) <= T::one()
}

impl<T: BaseFloat> Cylinder<T> {
    fn intersect_caps(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let mut xs = Vec::new();
        if self.closed && abs_diff_ne!(ray.direction.y, T::zero()) {
            for m in [self.minimum, self.maximum] {
                let t = (m - ray.origin.y) / ray.direction.y;
                if check_cap(ray, t) {
                    xs.push(Intersection::new(t, Shape::Cylinder(self.clone())));
                }
            }
        }
        xs
    }
}

impl<T: BaseFloat> Cylinder<T> {
    pub fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    pub fn material(&self) -> Option<Material<T>> {
        Some(self.material)
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let two = T::from(2).unwrap();
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);
        if abs_diff_eq!(a, T::zero()) {
            self.intersect_caps(ray)
        } else {
            let b = two * (ray.origin.x * ray.direction.x + ray.origin.z * ray.direction.z);
            let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - T::one();
            let disc = b.powi(2) - T::from(4).unwrap() * a * c;
            if disc < T::zero() {
                vec![]
            } else {
                let t0 = (-b - disc.sqrt()) / (two * a);
                let t1 = (-b + disc.sqrt()) / (two * a);
                let mut xs = Vec::new();
                let y0 = ray.origin.y + t0 * ray.direction.y;
                if self.minimum < y0 && y0 < self.maximum {
                    xs.push(Intersection::new(t0, Shape::Cylinder(self.clone())));
                }
                let y1 = ray.origin.y + t1 * ray.direction.y;
                if self.minimum < y1 && y1 < self.maximum {
                    xs.push(Intersection::new(t1, Shape::Cylinder(self.clone())));
                }
                xs.append(&mut self.intersect_caps(ray));
                xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
                xs
            }
        }
    }

    pub fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < T::one() && abs_diff_eq!(point.y, self.maximum) {
            Vector3::unit_y()
        } else if dist < T::one() && abs_diff_eq!(point.y, self.minimum) {
            -Vector3::unit_y()
        } else {
            Vector3::new(point.x, T::zero(), point.z)
        }
    }
}

mod tests {
    use crate::shape::cylinder;

    use super::*;
    use cgmath::{assert_relative_eq, Rad};
    use std::f32::consts::PI;

    #[test]
    fn local_intersect() {
        {
            let cylinder = Cylinder::default();
            let shape = Shape::Cylinder(cylinder.clone());
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(1., 0., 0.), Vector3::unit_y())),
                vec![]
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::origin(), Vector3::unit_y())),
                vec![]
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(
                    Point3::new(0., 0., -5.),
                    Vector3::new(1., 1., 1.).normalize()
                )),
                vec![]
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(1., 0., -5.), Vector3::unit_z())),
                vec![
                    Intersection::new(5., shape.clone()),
                    Intersection::new(5., shape.clone())
                ]
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape)
                ]
            );
            assert_relative_eq!(
                cylinder
                    .local_intersect(Ray::new(
                        Point3::new(0.5, 0., -5.),
                        Vector3::new(0.1, 1., 1.).normalize()
                    ))
                    .iter()
                    .map(|i| i.t)
                    .collect::<Vec<_>>()[..],
                [6.80798, 7.08872,],
                max_relative = 0.00001
            );
        }
        {
            let mut cylinder = Cylinder::default();
            cylinder.minimum = 1.;
            cylinder.maximum = 2.;
            assert_eq!(
                cylinder.local_intersect(Ray::new(
                    Point3::new(0., 1.5, 0.),
                    Vector3::new(0.1, 1., 0.).normalize()
                )),
                vec![],
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(0., 3., -5.), Vector3::unit_z())),
                vec![],
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z())),
                vec![],
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(0., 2., -5.), Vector3::unit_z())),
                vec![],
            );
            assert_eq!(
                cylinder.local_intersect(Ray::new(Point3::new(0., 1., -5.), Vector3::unit_z())),
                vec![],
            );
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(Point3::new(0., 1.5, -2.), Vector3::unit_z()))
                    .len(),
                2,
            );
        }
        {
            let mut cylinder = Cylinder::default();
            cylinder.minimum = 1.;
            cylinder.maximum = 2.;
            cylinder.closed = true;
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(Point3::new(0., 3., 0.), -Vector3::unit_y()))
                    .len(),
                2
            );
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(
                        Point3::new(0., 3., -2.),
                        -Vector3::new(0., -1., 2.).normalize()
                    ))
                    .len(),
                2
            );
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(
                        Point3::new(0., 4., -2.),
                        -Vector3::new(0., -1., 1.).normalize()
                    ))
                    .len(),
                2
            );
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(
                        Point3::new(0., 0., -2.),
                        -Vector3::new(0., 1., 2.).normalize()
                    ))
                    .len(),
                2
            );
            assert_eq!(
                cylinder
                    .local_intersect(Ray::new(
                        Point3::new(0., -1., -2.),
                        -Vector3::new(0., 1., 1.).normalize()
                    ))
                    .len(),
                2
            );
        }
    }

    #[test]
    fn local_normal_at() {
        {
            let cylinder = Cylinder::default();
            assert_eq!(
                cylinder.local_normal_at(Point3::new(1., 0., 0.)),
                Vector3::unit_x()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., 5., -1.)),
                -Vector3::unit_z()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., -2., 1.)),
                Vector3::unit_z()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(-1., 1., 0.)),
                -Vector3::unit_x()
            );
        }
        {
            let mut cylinder = Cylinder::default();
            cylinder.minimum = 1.;
            cylinder.maximum = 2.;
            cylinder.closed = true;
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., 1., 0.)),
                -Vector3::unit_y()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0.5, 1., 0.)),
                -Vector3::unit_y()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., 1., 0.5)),
                -Vector3::unit_y()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., 2., 0.)),
                Vector3::unit_y()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0.5, 2., 0.)),
                Vector3::unit_y()
            );
            assert_eq!(
                cylinder.local_normal_at(Point3::new(0., 2., 0.5)),
                Vector3::unit_y()
            );
        }
    }
}
