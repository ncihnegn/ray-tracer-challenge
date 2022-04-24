use crate::pattern::TraitPattern;
use cgmath::{BaseFloat, Matrix4, Point3};
use derive_more::Constructor;
use num_traits::cast;
use rgb::RGB;

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Stripe<T> {
    a: RGB<T>,
    b: RGB<T>,
    transform: Matrix4<T>,
}

impl<T: BaseFloat> TraitPattern<T> for Stripe<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn at(&self, point: Point3<T>) -> RGB<T> {
        let i: i32 = cast(point.x.floor()).unwrap();
        if i % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

mod tests {
    use super::*;
    use cgmath::{EuclideanSpace, SquareMatrix, Vector3};

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        let stripe = Stripe::new(white, black, Matrix4::identity());
        assert_eq!(stripe.at(Point3::origin()), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_y())), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_y() * 2.)), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_z())), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_z() * 2.)), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_x() * 0.9)), white);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_x())), black);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_x() * -0.1)), black);
        assert_eq!(stripe.at(Point3::from_vec(-Vector3::unit_x())), black);
        assert_eq!(stripe.at(Point3::from_vec(Vector3::unit_x() * -1.1)), white);
    }
}
