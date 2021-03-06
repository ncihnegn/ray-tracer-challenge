use crate::{
    bounds::Bounds,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, ShapeWeak, Triangle},
};
use cgmath::{BaseFloat, Matrix4, Point3, SquareMatrix, Vector3};

#[derive(Clone, derive_more::Constructor, Debug, derivative::Derivative)]
#[derivative(PartialEq)]
pub struct SmoothTriangle<T> {
    pub material: Material<T>,
    pub p1: Point3<T>,
    pub p2: Point3<T>,
    pub p3: Point3<T>,
    pub n1: Vector3<T>,
    pub n2: Vector3<T>,
    pub n3: Vector3<T>,
    #[derivative(PartialEq = "ignore")]
    pub parent: Option<ShapeWeak<T>>,
}

impl<T: BaseFloat> SmoothTriangle<T> {
    pub fn bounds(&self) -> Bounds<T> {
        Bounds::from_all_points(&[self.p1, self.p2, self.p3]).unwrap()
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        Triangle::from(self.p1, self.p2, self.p3)
            .local_intersect(ray)
            .iter()
            .map(|i| Intersection::new(i.t, Shape::SmoothTriangle(self.clone()), i.uv))
            .collect::<Vec<_>>()
    }

    pub fn local_normal_at(&self, _point: Point3<T>, uv: Option<(T, T)>) -> Vector3<T> {
        let (u, v) = uv.unwrap();
        self.n2 * u + self.n3 * v + self.n1 * (T::one() - u - v)
    }
}
mod tests {
    use super::*;
    use cgmath::{assert_relative_eq, EuclideanSpace};

    #[test]
    fn local_intersect() {
        let tri = SmoothTriangle::new(
            Material::default(),
            Point3::new(0., 1., 0.),
            Point3::new(-1., 0., 0.),
            Point3::new(1., 0., 0.),
            Vector3::unit_y(),
            -Vector3::unit_x(),
            Vector3::unit_x(),
            None,
        );
        let ray = Ray::new(Point3::new(-0.2, 0.3, -2.), Vector3::unit_z());
        let (u, v) = tri.local_intersect(ray)[0].uv.unwrap();
        assert_relative_eq!(u, 0.45);
        assert_relative_eq!(v, 0.25);
    }

    #[test]
    fn normal_at() {
        let tri = SmoothTriangle::new(
            Material::default(),
            Point3::new(0., 1., 0.),
            Point3::new(-1., 0., 0.),
            Point3::new(1., 0., 0.),
            Vector3::unit_y(),
            -Vector3::unit_x(),
            Vector3::unit_x(),
            None,
        );
        assert_relative_eq!(
            Shape::SmoothTriangle(tri)
                .normal_at(Point3::origin(), Some((0.45, 0.25)))
                .unwrap(),
            Vector3::new(-0.5547, 0.83205, 0.),
            max_relative = 0.00001
        );
    }

    #[test]
    fn precompute() {
        let i = Intersection::new(
            1.,
            Shape::SmoothTriangle(SmoothTriangle::new(
                Material::default(),
                Point3::new(0., 1., 0.),
                Point3::new(-1., 0., 0.),
                Point3::new(1., 0., 0.),
                Vector3::unit_y(),
                -Vector3::unit_x(),
                Vector3::unit_x(),
                None,
            )),
            Some((0.45, 0.25)),
        );
        let r = Ray::new(Point3::new(-0.2, 0.3, -2.), Vector3::unit_z());
        assert_relative_eq!(
            i.precompute(r, &vec![i.clone()]).unwrap().normalv,
            Vector3::new(-0.5547, 0.83205, 0.),
            max_relative = 0.00001
        );
    }
}
