use crate::ray::Ray;
use cgmath::{BaseFloat, EuclideanSpace, Matrix4, Point3};
use derive_more::Constructor;
use std::cmp::Ordering::Less;

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Bounds<T> {
    pub minimum: Point3<T>,
    pub maximum: Point3<T>,
}

fn check_axis<T: BaseFloat>(origin: T, direction: T, minimum: T, maximum: T) -> (T, T) {
    let (tmin_numerator, tmax_numerator) = (minimum - origin, maximum - origin);
    let (tmin, tmax) = (tmin_numerator / direction, tmax_numerator / direction);
    if tmin < tmax {
        (tmin, tmax)
    } else {
        (tmax, tmin)
    }
}

impl<T: BaseFloat> Bounds<T> {
    pub fn from_all_points(points: &[Point3<T>]) -> Option<Bounds<T>> {
        if points.is_empty() {
            None
        } else {
            Some(Bounds::new(
                Point3::new(
                    points
                        .iter()
                        .min_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Less))
                        .unwrap()
                        .x,
                    points
                        .iter()
                        .min_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(Less))
                        .unwrap()
                        .y,
                    points
                        .iter()
                        .min_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(Less))
                        .unwrap()
                        .z,
                ),
                Point3::new(
                    points
                        .iter()
                        .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(Less))
                        .unwrap()
                        .x,
                    points
                        .iter()
                        .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap_or(Less))
                        .unwrap()
                        .y,
                    points
                        .iter()
                        .max_by(|a, b| a.z.partial_cmp(&b.z).unwrap_or(Less))
                        .unwrap()
                        .z,
                ),
            ))
        }
    }

    pub fn all_points(&self) -> [Point3<T>; 8] {
        [
            self.minimum,
            Point3::new(self.maximum.x, self.minimum.y, self.minimum.z),
            Point3::new(self.minimum.x, self.maximum.y, self.minimum.z),
            Point3::new(self.minimum.x, self.minimum.y, self.maximum.z),
            Point3::new(self.maximum.x, self.maximum.y, self.minimum.z),
            Point3::new(self.maximum.x, self.minimum.y, self.maximum.z),
            Point3::new(self.minimum.x, self.maximum.y, self.maximum.z),
            self.maximum,
        ]
    }

    pub fn check_axes(&self, ray: Ray<T>) -> Bounds<T> {
        let (xmin, xmax) = check_axis(
            ray.origin.x,
            ray.direction.x,
            self.minimum.x,
            self.maximum.x,
        );
        let (ymin, ymax) = check_axis(
            ray.origin.y,
            ray.direction.y,
            self.minimum.y,
            self.maximum.y,
        );
        let (zmin, zmax) = check_axis(
            ray.origin.z,
            ray.direction.z,
            self.minimum.z,
            self.maximum.z,
        );
        Bounds::new(Point3::new(xmin, ymin, zmin), Point3::new(xmax, ymax, zmax))
    }

    pub fn transform(&self, transform: Matrix4<T>) -> Vec<Point3<T>> {
        self.all_points()
            .iter()
            .map(|p| Point3::from_homogeneous(transform * p.to_homogeneous()))
            .collect::<Vec<_>>()
    }

    pub fn minmax(&self) -> (T, T) {
        let (min, max) = (self.minimum, self.maximum);
        (min.x.max(min.y).max(min.z), max.x.min(max.y).min(max.z))
    }

    pub fn is_intersected_with(&self, ray: Ray<T>) -> bool {
        let (tmin, tmax) = self.check_axes(ray).minmax();
        tmin <= tmax
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::Vector3;

    #[test]
    fn is_intersected_with() {
        {
            let b = Bounds::<f32>::new(Point3::new(5., -2., 0.), Point3::new(11., 4., 7.));
            assert_eq!(
                b.is_intersected_with(Ray::new(Point3::new(15., 1., 2.), -Vector3::unit_x())),
                true
            );
            assert_eq!(
                b.is_intersected_with(Ray::new(Point3::new(-5., -1., 4.), Vector3::unit_x())),
                true
            );
            assert_eq!(
                b.is_intersected_with(Ray::new(Point3::new(7., 6., 5.), -Vector3::unit_y())),
                true
            );
        }
        {
            let b = Bounds::<f32>::new(Point3::new(8., -2., -2.), Point3::new(12., 2., 2.));
            assert_eq!(
                b.is_intersected_with(Ray::new(Point3::new(10., 0., -10.), Vector3::unit_z())),
                true
            );
        }
    }
}
