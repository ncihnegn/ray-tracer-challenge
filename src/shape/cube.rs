use crate::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, TraitShape},
};
use cgmath::{BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Cube<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

impl<T: BaseFloat + Default> Default for Cube<T> {
    fn default() -> Cube<T> {
        Cube::<T> {
            transform: Matrix4::identity(),
            material: Material::default(),
        }
    }
}

fn check_axis<T: BaseFloat>(origin: T, direction: T) -> (T, T) {
    let tmin_numerator = T::from(-1).unwrap() - origin;
    let tmax_numerator = T::one() - origin;
    let (tmin, tmax) = if direction.abs() >= T::epsilon() {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * T::infinity(),
            tmax_numerator * T::infinity(),
        )
    };
    if tmin < tmax {
        (tmin, tmax)
    } else {
        (tmax, tmin)
    }
}

impl<T: BaseFloat> TraitShape<T> for Cube<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn material(&self) -> Option<Material<T>> {
        Some(self.material)
    }

    fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin < tmax {
            vec![
                Intersection::new(tmin, Shape::Cube(self.clone())),
                Intersection::new(tmax, Shape::Cube(self.clone())),
            ]
        } else {
            vec![]
        }
    }

    fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        let maxc = point.x.abs().max(point.y.abs()).max(point.z.abs());
        if maxc == point.x.abs() {
            Vector3::unit_x() * point.x
        } else if maxc == point.y.abs() {
            Vector3::unit_y() * point.y
        } else {
            Vector3::unit_z() * point.z
        }
    }
}

mod tests {
    use super::*;
    use cgmath::{assert_relative_eq, Rad};
    use std::f32::consts::PI;

    #[test]
    fn local_intersect() {
        let cube = Cube::default();
        let shape = Shape::Cube(cube.clone());
        {
            let ray = Ray::new(Point3::new(5., 0.5, 0.), -Vector3::unit_x());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(-5., 0.5, 0.), Vector3::unit_x());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(0.5, 5., 0.), -Vector3::unit_y());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(0.5, -5., 0.), Vector3::unit_y());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(0.5, 0., 5.), -Vector3::unit_z());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(0.5, 0., -5.), Vector3::unit_z());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(4., shape.clone()),
                    Intersection::new(6., shape.clone())
                ]
            );
        }
        {
            let ray = Ray::new(Point3::new(0., 0.5, 0.), Vector3::unit_z());
            assert_eq!(
                cube.local_intersect(ray),
                vec![
                    Intersection::new(-1., shape.clone()),
                    Intersection::new(1., shape)
                ]
            );
        }
        {
            let ray = Ray::new(
                Point3::new(-2., 0., 0.),
                Vector3::new(0.2673, 0.5345, 0.8018),
            );
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
        {
            let ray = Ray::new(
                Point3::new(0., -2., 0.),
                Vector3::new(0.8018, 0.2673, 0.5345),
            );
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
        {
            let ray = Ray::new(
                Point3::new(0., 0., -2.),
                Vector3::new(0.5345, 0.8018, 0.2673),
            );
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
        {
            let ray = Ray::new(Point3::new(2., 0., 2.), -Vector3::unit_z());
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
        {
            let ray = Ray::new(Point3::new(0., 2., 2.), -Vector3::unit_y());
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
        {
            let ray = Ray::new(Point3::new(2., 2., 0.), -Vector3::unit_x());
            assert_eq!(cube.local_intersect(ray), vec![]);
        }
    }

    #[test]
    fn local_normal_at() {
        let cube = Cube::default();
        assert_eq!(
            cube.local_normal_at(Point3::new(1., 0.5, -0.8)),
            Vector3::unit_x()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(-1., -0.2, 0.9)),
            -Vector3::unit_x()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(-0.4, 1., -0.1)),
            Vector3::unit_y()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(0.3, -1., -0.7)),
            -Vector3::unit_y()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(-0.6, 0.3, 1.)),
            Vector3::unit_z()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(0.4, 0.4, -1.)),
            -Vector3::unit_z()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(1., 1., 1.)),
            Vector3::unit_x()
        );
        assert_eq!(
            cube.local_normal_at(Point3::new(-1., -1., -1.)),
            -Vector3::unit_x()
        );
    }
}
