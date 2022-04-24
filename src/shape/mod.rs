pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod obj_file;
pub mod plane;
pub mod smooth_triangle;
pub mod sphere;
pub mod triangle;

use crate::{
    bounds::Bounds,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane,
        smooth_triangle::SmoothTriangle, sphere::Sphere, triangle::Triangle,
    },
};
use cgmath::{
    BaseFloat, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix, Vector3,
};
use derivative::Derivative;
use derive_more::Constructor;
use enum_as_inner::EnumAsInner;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Clone, Debug, EnumAsInner, PartialEq)]
pub enum Shape<T> {
    Cone(Cone<T>),
    Cube(Cube<T>),
    Cylinder(Cylinder<T>),
    Group(Group<T>),
    Plane(Plane<T>),
    SmoothTriangle(SmoothTriangle<T>),
    Sphere(Sphere<T>),
    Triangle(Triangle<T>),
}

impl<T: BaseFloat> Shape<T> {
    pub fn transform(&self) -> Matrix4<T> {
        match self {
            Shape::Cone(c) => c.transform(),
            Shape::Cube(c) => c.transform(),
            Shape::Cylinder(c) => c.transform(),
            Shape::Group(g) => g.transform(),
            Shape::Plane(p) => p.transform(),
            Shape::SmoothTriangle(s) => s.transform(),
            Shape::Sphere(s) => s.transform(),
            Shape::Triangle(t) => t.transform(),
        }
    }

    pub fn material(&self) -> Option<Material<T>> {
        match self {
            Shape::Cone(c) => Some(c.material()),
            Shape::Cube(c) => Some(c.material()),
            Shape::Cylinder(c) => Some(c.material()),
            Shape::Group(_) => None,
            Shape::Plane(p) => Some(p.material()),
            Shape::SmoothTriangle(s) => Some(s.material()),
            Shape::Sphere(s) => Some(s.material()),
            Shape::Triangle(t) => Some(t.material()),
        }
    }

    pub fn bounds(&self) -> Option<Bounds<T>> {
        match self {
            Shape::Cone(c) => Some(c.bounds()),
            Shape::Cube(c) => Some(c.bounds()),
            Shape::Cylinder(c) => Some(c.bounds()),
            Shape::Group(g) => g.bounds(),
            Shape::Plane(p) => Some(p.bounds()),
            Shape::SmoothTriangle(s) => Some(s.bounds()),
            Shape::Sphere(s) => Some(s.bounds()),
            Shape::Triangle(t) => Some(t.bounds()),
        }
    }

    fn local_normal_at(&self, point: Point3<T>, uv: Option<(T, T)>) -> Vector3<T> {
        match self {
            Shape::Cone(c) => c.local_normal_at(point),
            Shape::Cube(c) => c.local_normal_at(point),
            Shape::Cylinder(c) => c.local_normal_at(point),
            Shape::Group(_) => {
                panic!("The local_normal_at() is not supposed to by called on Shape::Group.")
            }
            Shape::Plane(p) => p.local_normal_at(point),
            Shape::SmoothTriangle(s) => s.local_normal_at(point, uv),
            Shape::Sphere(s) => s.local_normal_at(point),
            Shape::Triangle(t) => t.local_normal_at(point),
        }
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        match self {
            Shape::Cone(c) => c.local_intersect(ray),
            Shape::Cube(c) => c.local_intersect(ray),
            Shape::Cylinder(c) => c.local_intersect(ray),
            Shape::Group(g) => g.local_intersect(ray),
            Shape::Plane(p) => p.local_intersect(ray),
            Shape::SmoothTriangle(s) => s.local_intersect(ray),
            Shape::Sphere(s) => s.local_intersect(ray),
            Shape::Triangle(t) => t.local_intersect(ray),
        }
    }

    pub fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if let Some(i) = self.transform().invert() {
            self.local_intersect(ray.transform(i))
        } else {
            Vec::new()
        }
    }

    pub fn normal_at(&self, point: Point3<T>, uv: Option<(T, T)>) -> Option<Vector3<T>> {
        self.transform().invert().map(|i| {
            (i.transpose()
                * self
                    .local_normal_at(
                        Point3::from_vec((i * point.to_homogeneous()).truncate()),
                        uv,
                    )
                    .extend(T::zero()))
            .truncate()
            .normalize()
        })
    }
}

pub fn reflect<T: BaseFloat>(v: Vector3<T>, normal: Vector3<T>) -> Vector3<T> {
    v - normal * T::from(2).unwrap() * v.dot(normal)
}

#[derive(Clone, Constructor, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct ShapeWrapper<T> {
    pub shape: Shape<T>,
    #[derivative(PartialEq = "ignore")]
    pub parent: Option<Weak<RefCell<ShapeWrapper<T>>>>,
}

pub type ShapeLink<T> = Rc<RefCell<ShapeWrapper<T>>>;

pub fn get_link<T>(shape: Shape<T>) -> ShapeLink<T> {
    Rc::new(RefCell::new(ShapeWrapper::new(shape, None)))
}

pub fn get_link_with_parent<T>(shape: Shape<T>, parent: &ShapeLink<T>) -> ShapeLink<T> {
    Rc::new(RefCell::new(ShapeWrapper::new(
        shape,
        Some(Rc::downgrade(parent)),
    )))
}

impl<T: BaseFloat> ShapeWrapper<T> {
    pub fn world_to_object(&self, point: Point3<T>) -> Option<Point3<T>> {
        let pp = Some(point);
        let o = self.parent.as_ref().map_or(pp, |weak| {
            weak.upgrade()
                .map_or(pp, |rc| rc.borrow().world_to_object(point))
        });
        self.shape
            .transform()
            .invert()
            .and_then(|i| o.map(|p| Point3::from_vec((i * p.to_homogeneous()).truncate())))
    }

    fn normal_to_world(&self, normal: Vector3<T>) -> Option<Vector3<T>> {
        let ov = self.shape.transform().invert().map(|i| {
            (i.transpose() * normal.extend(T::zero()))
                .truncate()
                .normalize()
        });
        self.parent.as_ref().map_or(ov, |weak| {
            weak.upgrade()
                .map_or(ov, |rc| ov.and_then(|v| rc.borrow().normal_to_world(v)))
        })
    }

    pub fn normal_at(&self, world_point: Point3<T>, uv: Option<(T, T)>) -> Option<Vector3<T>> {
        self.world_to_object(world_point)
            .map(|local_point| self.shape.local_normal_at(local_point, uv))
            .map(|local_normal| self.normal_to_world(local_normal).unwrap())
    }
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
            .normal_at(Point3::new(0., 1.70711, -0.70711), None)
            .unwrap(),
            Vector3::new(0., 0.70711, -0.70711),
            max_relative = 0.00001,
        );
        assert_relative_eq!(
            Shape::Sphere(Sphere::new(
                Matrix4::from_nonuniform_scale(1., 0.5, 1.) * Matrix4::from_angle_z(Rad(PI / 5.)),
                Material::default(),
            ))
            .normal_at(2.0_f32.sqrt().recip() * Point3::new(0., 1., -1.), None)
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
