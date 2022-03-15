use crate::material::Material;
use cgmath::{BaseFloat, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix, Vector3};
use derive_more::Constructor;

#[derive(Constructor, Clone, Debug, PartialEq)]
pub struct Sphere<T> {
    pub transform: Matrix4<T>,
    pub material: Material<T>,
}

pub fn normal_at<T: BaseFloat>(s: Matrix4<T>, p: Point3<T>) -> Option<Vector3<T>> {
    s.invert().map(|m| {
        let object_point = m * p.to_homogeneous();
        (m.transpose() * object_point).truncate().normalize()
    })
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
        assert_eq!(
            normal_at(Matrix4::from_scale(1.), Point3::new(1., 0., 0.)),
            Some(Vector3::unit_x())
        );
        assert_eq!(
            normal_at(Matrix4::from_scale(1.), Point3::new(0., 1., 0.)),
            Some(Vector3::unit_y())
        );
        assert_eq!(
            normal_at(Matrix4::from_scale(1.), Point3::new(0., 0., 1.)),
            Some(Vector3::unit_z())
        );
        let d = 3.0_f32.sqrt().recip();
        assert_relative_eq!(
            normal_at(Matrix4::from_scale(1.), Point3::new(d, d, d)).unwrap(),
            Vector3::new(d, d, d)
        );
        assert_relative_eq!(
            normal_at(
                Matrix4::from_translation(Vector3::unit_y()),
                Point3::new(0., 1.70711, -0.70711)
            )
            .unwrap(),
            Vector3::new(0., 0.70711, -0.70711),
            max_relative = 0.00001,
        );
        let t = 2.0_f32.sqrt().recip();
        assert_relative_eq!(
            normal_at(
                Matrix4::from_nonuniform_scale(1., 0.5, 1.)
                    * Matrix4::from_axis_angle(Vector3::unit_z(), Rad(PI / 5.)),
                Point3::new(0., t, -t)
            )
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
                Vector3::new(0., -1., 0.),
                Vector3::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.)
            ),
            Vector3::unit_x()
        );
    }
}
