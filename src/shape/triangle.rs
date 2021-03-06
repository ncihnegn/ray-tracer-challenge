use crate::{
    bounds::Bounds,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, ShapeWeak},
};
use cgmath::{
    abs_diff_ne, BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3,
};

#[derive(Clone, derive_more::Constructor, Debug, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct Triangle<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
    pub p1: Point3<T>,
    pub p2: Point3<T>,
    pub p3: Point3<T>,
    pub e1: Vector3<T>,
    pub e2: Vector3<T>,
    pub normal: Vector3<T>,
    #[derivative(PartialEq = "ignore")]
    pub parent: Option<ShapeWeak<T>>,
}

impl<T: BaseFloat> Default for Triangle<T> {
    fn default() -> Triangle<T> {
        Triangle::from(Point3::origin(), Point3::origin(), Point3::origin())
    }
}

impl<T: BaseFloat> Triangle<T> {
    pub fn from(p1: Point3<T>, p2: Point3<T>, p3: Point3<T>) -> Triangle<T> {
        let (e1, e2) = (p2 - p1, p3 - p1);
        Triangle {
            transform: Matrix4::identity(),
            material: Material::default(),
            p1,
            p2,
            p3,
            e1,
            e2,
            normal: e2.cross(e1).normalize(),
            parent: None,
        }
    }
}

impl<T: BaseFloat> Triangle<T> {
    pub fn bounds(&self) -> Bounds<T> {
        Bounds::from_all_points(&[self.p1, self.p2, self.p3]).unwrap()
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);
        if abs_diff_ne!(det, T::epsilon()) {
            let f = T::one() / det;
            let p1_to_origin = ray.origin - self.p1;
            let u = f * p1_to_origin.dot(dir_cross_e2);
            if T::zero() <= u && u <= T::one() {
                let origin_cross_e1 = p1_to_origin.cross(self.e1);
                let v = f * ray.direction.dot(origin_cross_e1);
                if v >= T::zero() && (u + v) <= T::one() {
                    let t = f * self.e2.dot(origin_cross_e1);
                    return vec![Intersection::new(
                        t,
                        Shape::Triangle(self.clone()),
                        Some((u, v)),
                    )];
                }
            }
        }
        vec![]
    }

    pub fn local_normal_at(&self, _point: Point3<T>) -> Vector3<T> {
        self.normal
    }
}

mod tests {
    use super::*;
    use cgmath::assert_relative_eq;

    #[test]
    fn local_intersect() {
        let t = Triangle::from(
            Point3::new(0., 1., 0.),
            Point3::new(-1., 0., 0.),
            Point3::new(1., 0., 0.),
        );
        let shape = Shape::Triangle(t.clone());
        assert_eq!(
            t.local_intersect(Ray::new(Point3::new(0., -1., -2.), Vector3::unit_y())),
            vec![]
        );
        assert_eq!(
            t.local_intersect(Ray::new(Point3::new(1., 1., -2.), Vector3::unit_z())),
            vec![]
        );
        assert_eq!(
            t.local_intersect(Ray::new(Point3::new(-1., 1., -2.), Vector3::unit_z())),
            vec![]
        );
        assert_eq!(
            t.local_intersect(Ray::new(Point3::new(0., -1., -2.), Vector3::unit_z())),
            vec![]
        );
        assert_eq!(
            t.local_intersect(Ray::new(Point3::new(0., 0.5, -2.), Vector3::unit_z())),
            vec![Intersection::new(2., shape, Some((0.25, 0.25)))]
        );
    }

    #[test]
    fn local_normal_at() {
        let t = Triangle::from(
            Point3::new(0., 1., 0.),
            Point3::new(-1., 0., 0.),
            Point3::new(1., 0., 0.),
        );
        assert_eq!(t.local_normal_at(Point3::new(0., 0.5, 0.)), t.normal);
        assert_eq!(t.local_normal_at(Point3::new(-0.5, 0.75, 0.)), t.normal);
        assert_eq!(t.local_normal_at(Point3::new(0.5, 0.25, 0.)), t.normal);
    }
}
