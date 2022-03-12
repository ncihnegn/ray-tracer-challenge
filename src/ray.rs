use crate::intersection::Intersection;
use cgmath::{BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};

#[derive(Debug, PartialEq)]
pub(crate) struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
}

impl<T: BaseFloat> Ray<T> {
    pub fn new(origin: Point3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray::<T> { origin, direction }
    }

    // Find the position after time.
    fn position(&self, time: T) -> Point3<T> {
        self.origin + self.direction * time
    }

    // Find intersection with the unit sphere.
    fn intersect_unit(&self) -> Vec<T> {
        let sphere_to_ray = self.origin.to_vec();
        let a = self.direction.dot(self.direction);
        let two = T::from(2).unwrap();
        let b = self.direction.dot(sphere_to_ray) * two;
        let c = sphere_to_ray.dot(sphere_to_ray) - T::one();
        let discriminant = b * b - T::from(4).unwrap() * a * c;
        match discriminant {
            d if d > T::zero() => vec![(-b - d.sqrt()) / (two * a), (-b + d.sqrt()) / (two * a)],
            d if d == T::zero() => vec![-b / (two * a)],
            _ => vec![],
        }
    }

    fn transform(&self, transform: Matrix4<T>) -> Ray<T> {
        Ray::<T> {
            origin: Point3::from_homogeneous(transform * self.origin.to_homogeneous()),
            direction: (transform * self.direction.extend(T::zero())).truncate(),
        }
    }

    fn intersect(&self, transform: Matrix4<T>) -> Option<Vec<Intersection<T>>> {
        transform.invert().map(|m| {
            self.transform(m)
                .intersect_unit()
                .iter()
                .map(|&t| Intersection::<T> { transform, t })
                .collect()
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn position() {
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(0.),
            Point3::new(2., 3., 4.)
        );
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(-1.),
            Point3::new(1., 3., 4.)
        );
        assert_eq!(
            Ray::new(Point3::new(2., 3., 4.), Vector3::unit_x()).position(2.5),
            Point3::new(4.5, 3., 4.)
        );
    }

    #[test]
    fn intersect_unit() {
        // A ray intersecs the sphere at two points
        assert_eq!(
            Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()).intersect_unit(),
            vec![4., 6.]
        );
        // A ray intersecs the sphere at a tangent.
        assert_eq!(
            Ray::new(Point3::new(0., 1., -5.), Vector3::unit_z()).intersect_unit(),
            vec![5.]
        );
        // A ray misses the sphere.
        assert!(Ray::new(Point3::new(0., 2., -5.), Vector3::unit_z())
            .intersect_unit()
            .is_empty());

        // A ray originates inside the sphere.
        assert_eq!(
            Ray::<f32>::new(Point3::origin(), Vector3::unit_z(),).intersect_unit(),
            vec![-1., 1.]
        );
        // A ray is in front of the sphere.
        assert_eq!(
            Ray::new(Point3::new(0., 0., 5.), Vector3::unit_z()).intersect_unit(),
            vec![-6., -4.]
        );
    }

    #[test]
    fn transform() {
        assert_eq!(
            Ray::<f32>::new(Point3::new(1., 2., 3.), Vector3::unit_y(),)
                .transform(Matrix4::from_translation(Vector3::new(3., 4., 5.))),
            Ray::<f32>::new(Point3::new(4., 6., 8.), Vector3::unit_y(),)
        );
        assert_eq!(
            Ray::<f32>::new(Point3::new(1., 2., 3.), Vector3::unit_y(),)
                .transform(Matrix4::from_nonuniform_scale(2., 3., 4.)),
            Ray::<f32>::new(Point3::new(2., 6., 12.), Vector3::new(0., 3., 0.),)
        );
    }

    #[test]
    fn intersect() {
        let transform = Matrix4::from_scale(2.);
        assert_eq!(
            Ray::<f32>::new(Point3::new(0., 0., -5.), Vector3::unit_z(),).intersect(transform),
            Some(vec![
                Intersection::new(transform, 3.),
                Intersection::new(transform, 7.)
            ])
        );
        let translation = Matrix4::from_translation(Vector3::new(5., 0., 0.));
        assert_eq!(
            Ray::<f32>::new(Point3::new(0., 0., -5.), Vector3::unit_z(),).intersect(translation),
            Some(vec![])
        );
    }
}
