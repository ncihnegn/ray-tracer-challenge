pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod plane;
pub mod sphere;

use crate::{
    bounds::Bounds,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere,
    },
};
use cgmath::{
    BaseFloat, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix, Vector3,
};
use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, EnumAsInner, PartialEq)]
pub enum Shape<T> {
    Cone(Cone<T>),
    Cube(Cube<T>),
    Cylinder(Cylinder<T>),
    Group(Group<T>),
    Plane(Plane<T>),
    Sphere(Sphere<T>),
}

impl<T: BaseFloat> Shape<T> {
    pub fn transform(&self) -> Matrix4<T> {
        match self {
            Shape::Cone(c) => c.transform(),
            Shape::Cube(c) => c.transform(),
            Shape::Cylinder(c) => c.transform(),
            Shape::Group(g) => g.transform(),
            Shape::Plane(p) => p.transform(),
            Shape::Sphere(s) => s.transform(),
        }
    }

    pub fn material(&self) -> Option<Material<T>> {
        match self {
            Shape::Cone(c) => Some(c.material()),
            Shape::Cube(c) => Some(c.material()),
            Shape::Cylinder(c) => Some(c.material()),
            Shape::Group(_) => None,
            Shape::Plane(p) => Some(p.material()),
            Shape::Sphere(s) => Some(s.material()),
        }
    }

    pub fn bounds(&self) -> Option<Bounds<T>> {
        match self {
            Shape::Cone(c) => Some(c.bounds()),
            Shape::Cube(c) => Some(c.bounds()),
            Shape::Cylinder(c) => Some(c.bounds()),
            Shape::Group(g) => g.bounds(),
            Shape::Plane(p) => Some(p.bounds()),
            Shape::Sphere(s) => Some(s.bounds()),
        }
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        match self {
            Shape::Cone(c) => c.local_normal_at(point),
            Shape::Cube(c) => c.local_normal_at(point),
            Shape::Cylinder(c) => c.local_normal_at(point),
            Shape::Group(_) => {
                panic!("The local_normal_at() is not supposed to by called on Shape::Group.")
            }
            Shape::Plane(p) => p.local_normal_at(point),
            Shape::Sphere(s) => s.local_normal_at(point),
        }
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        match self {
            Shape::Cone(c) => c.local_intersect(ray),
            Shape::Cube(c) => c.local_intersect(ray),
            Shape::Cylinder(c) => c.local_intersect(ray),
            Shape::Group(g) => g.local_intersect(ray),
            Shape::Plane(p) => p.local_intersect(ray),
            Shape::Sphere(s) => s.local_intersect(ray),
        }
    }

    pub fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if let Some(i) = self.transform().invert() {
            self.local_intersect(ray.transform(i))
        } else {
            Vec::new()
        }
    }

    pub fn normal_at(&self, point: Point3<T>) -> Option<Vector3<T>> {
        self.transform().invert().map(|i| {
            (i.transpose()
                * self
                    .local_normal_at(Point3::from_vec((i * point.to_homogeneous()).truncate()))
                    .extend(T::zero()))
            .truncate()
            .normalize()
        })
    }
}

pub fn reflect<T: BaseFloat>(v: Vector3<T>, normal: Vector3<T>) -> Vector3<T> {
    v - normal * T::from(2).unwrap() * v.dot(normal)
}

mod tests {
    use super::*;
    use cgmath::{assert_relative_eq, Rad};
    use std::f32::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn normal_at() {
        assert_relative_eq!(
            Shape::Sphere(Sphere::new(
                Matrix4::from_translation(Vector3::unit_y()),
                Material::default(),
            ))
            .normal_at(Point3::new(0., 1.70711, -0.70711))
            .unwrap(),
            Vector3::new(0., 0.70711, -0.70711),
            max_relative = 0.00001,
        );
        assert_relative_eq!(
            Shape::Sphere(Sphere::new(
                Matrix4::from_nonuniform_scale(1., 0.5, 1.) * Matrix4::from_angle_z(Rad(PI / 5.)),
                Material::default(),
            ))
            .normal_at(2.0_f32.sqrt().recip() * Point3::new(0., 1., -1.))
            .unwrap(),
            Vector3::new(0., 0.97014, -0.24254),
            max_relative = 0.0001,
        );
    }

    #[test]
    fn reflection() {
        assert_relative_eq!(
            reflect(Vector3::new(1., -1., 0.), Vector3::unit_y()),
            Vector3::new(1., 1., 0.)
        );
        assert_relative_eq!(
            reflect(
                -Vector3::unit_y(),
                Vector3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.)
            ),
            Vector3::unit_x()
        );
    }
}
