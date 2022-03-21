use crate::intersection::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use cgmath::{
    BaseFloat, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix, Vector3,
};
use derive_more::Constructor;
use enum_as_inner::EnumAsInner;

#[derive(Clone, Copy, Debug, EnumAsInner, PartialEq)]
pub enum Shape<T> {
    Plane(Plane<T>),
    Sphere(Sphere<T>),
}

impl<T: BaseFloat> Shape<T> {
    pub fn transform(&self) -> Matrix4<T> {
        match self {
            Shape::Plane(p) => p.transform(),
            Shape::Sphere(s) => s.transform(),
        }
    }

    pub fn material(&self) -> Material<T> {
        match self {
            Shape::Plane(p) => p.material(),
            Shape::Sphere(s) => s.material(),
        }
    }

    pub fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        match self {
            Shape::Plane(p) => p.intersect(ray),
            Shape::Sphere(s) => s.intersect(ray),
        }
    }

    pub fn normal_at(&self, point: Point3<T>) -> Option<Vector3<T>> {
        match self {
            Shape::Plane(p) => p.normal_at(point),
            Shape::Sphere(s) => s.normal_at(point),
        }
    }
}

pub trait TraitShape<T: BaseFloat> {
    fn transform(&self) -> Matrix4<T>;
    fn material(&self) -> Material<T>;
    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>>;
    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T>;

    fn intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if let Some(i) = self.transform().invert() {
            self.local_intersect(ray.transform(i))
        } else {
            Vec::new()
        }
    }

    fn normal_at(&self, point: Point3<T>) -> Option<Vector3<T>> {
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

#[derive(Constructor, Copy, Clone, Debug, PartialEq)]
pub struct Sphere<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T: BaseFloat + Default> Default for Sphere<T> {
    fn default() -> Sphere<T> {
        Sphere::<T> {
            transform: Matrix4::from_scale(T::one()),
            material: Material::<T>::default(),
        }
    }
}

impl<T: BaseFloat> TraitShape<T> for Sphere<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Material<T> {
        self.material
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let sphere_to_ray = ray.origin.to_vec();
        let a = ray.direction.dot(ray.direction);
        let two = T::from(2).unwrap();
        let b = ray.direction.dot(sphere_to_ray) * two;
        let c = sphere_to_ray.dot(sphere_to_ray) - T::one();
        let discriminant = b * b - T::from(4).unwrap() * a * c;
        match discriminant {
            d if d > T::zero() => vec![
                Intersection::new((-b - d.sqrt()) / (two * a), Shape::Sphere(*self)),
                Intersection::new((-b + d.sqrt()) / (two * a), Shape::Sphere(*self)),
            ],
            d if d == T::zero() => vec![Intersection::new(-b / (two * a), Shape::Sphere(*self))],
            _ => vec![],
        }
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        point.to_vec()
    }
}

#[derive(Constructor, Copy, Clone, Debug, PartialEq)]
pub struct Plane<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T: BaseFloat + Default> Default for Plane<T> {
    fn default() -> Plane<T> {
        Plane::<T> {
            transform: Matrix4::from_scale(T::one()),
            material: Material::<T>::default(),
        }
    }
}

impl<T: BaseFloat> TraitShape<T> for Plane<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Material<T> {
        self.material
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if ray.direction.y.abs() < T::epsilon() {
            Vec::new()
        } else {
            vec![Intersection::new(
                -ray.origin.y / ray.direction.y,
                Shape::Plane(*self),
            )]
        }
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        Vector3::unit_y()
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
    fn normal() {
        {
            let sphere = Sphere::default();
            {
                let v = Vector3::unit_x();
                assert_eq!(sphere.normal_at(Point3::from_vec(v)), Some(v));
            }
            {
                let v = Vector3::unit_y();
                assert_eq!(sphere.normal_at(Point3::from_vec(v)), Some(v));
            }
            {
                let v = Vector3::unit_z();
                assert_eq!(sphere.normal_at(Point3::from_vec(v)), Some(v));
            }
            {
                let d = 3.0_f32.sqrt().recip();
                let v = Vector3::new(d, d, d);
                assert_relative_eq!(sphere.normal_at(Point3::from_vec(v)).unwrap(), v);
            }
        }
        assert_relative_eq!(
            Sphere::new(
                Matrix4::from_translation(Vector3::unit_y()),
                Material::default()
            )
            .normal_at(Point3::new(0., 1.70711, -0.70711))
            .unwrap(),
            Vector3::new(0., 0.70711, -0.70711),
            max_relative = 0.00001,
        );
        {
            let t = 2.0_f32.sqrt().recip();
            assert_relative_eq!(
                Sphere::new(
                    Matrix4::from_nonuniform_scale(1., 0.5, 1.)
                        * Matrix4::from_angle_z(Rad(PI / 5.)),
                    Material::default()
                )
                .normal_at(Point3::new(0., t, -t))
                .unwrap(),
                Vector3::new(0., 0.97014, -0.24254),
                max_relative = 0.0001,
            );
        }
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
