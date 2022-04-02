pub mod checker;
pub mod gradient;
pub mod ring;
pub mod stripe;
pub mod test;

use crate::{
    material::Material,
    pattern::{checker::Checker, gradient::Gradient, ring::Ring, stripe::Stripe, test::Test},
    shape::Shape,
};
use cgmath::{BaseFloat, EuclideanSpace, Matrix4, Point3, SquareMatrix};
use rgb::RGB;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pattern<T> {
    Solid(RGB<T>),
    Stripe(Stripe<T>),
    Gradient(Gradient<T>),
    Ring(Ring<T>),
    Checker(Checker<T>),
    Test(Test<T>),
}

impl<T: BaseFloat> Pattern<T> {
    pub fn at(&self, point: Point3<T>) -> RGB<T> {
        match self {
            Pattern::Solid(s) => s.at(point),
            Pattern::Stripe(s) => s.at(point),
            Pattern::Gradient(s) => s.at(point),
            Pattern::Ring(s) => s.at(point),
            Pattern::Checker(s) => s.at(point),
            Pattern::Test(s) => s.at(point),
        }
    }
}

pub trait TraitPattern<T: BaseFloat> {
    fn transform(&self) -> Matrix4<T>;

    fn at(&self, point: Point3<T>) -> RGB<T>;

    fn at_shape(&self, object: Shape<T>, world_point: Point3<T>) -> RGB<T> {
        let object_point = object.transform().invert().unwrap() * world_point.to_homogeneous();
        let pattern_point =
            Point3::from_vec((self.transform().invert().unwrap() * object_point).truncate());
        self.at(pattern_point)
    }
}

impl<T: BaseFloat> TraitPattern<T> for RGB<T> {
    fn transform(&self) -> Matrix4<T> {
        Matrix4::identity()
    }

    fn at(&self, _point: Point3<T>) -> RGB<T> {
        *self
    }
}

mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use cgmath::Vector3;

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        {
            let test = Test::new(Matrix4::identity());
            assert_eq!(test.at(Point3::origin()), black);
            assert_eq!(test.at(Point3::new(1., 1., 1.)), white);
        }
    }

    #[test]
    fn at_shape() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        {
            let object = Shape::Sphere(Sphere::new(Matrix4::from_scale(2.), Material::default()));
            let pattern = Stripe::new(white, black, Matrix4::identity());
            assert_eq!(pattern.at_shape(object, Point3::new(1.5, 0., 0.)), white);
        }
        {
            let object = Shape::Sphere(Sphere::default());
            let pattern = Stripe::new(white, black, Matrix4::from_scale(2.));
            assert_eq!(pattern.at_shape(object, Point3::new(1.5, 0., 0.)), white);
        }
        {
            let object = Shape::Sphere(Sphere::new(Matrix4::from_scale(2.), Material::default()));
            let pattern = Stripe::new(
                white,
                black,
                Matrix4::from_translation(Vector3::unit_x() * 0.5),
            );
            assert_eq!(pattern.at_shape(object, Point3::new(2.5, 0., 0.)), white);
        }
        {
            let object = Shape::Sphere(Sphere::new(Matrix4::from_scale(2.), Material::default()));
            let pattern = Test::new(Matrix4::identity());
            assert_eq!(
                pattern.at_shape(object, Point3::new(2., 3., 4.)),
                RGB::new(1., 1.5, 2.)
            );
        }
        {
            let object = Shape::Sphere(Sphere::new(Matrix4::from_scale(2.), Material::default()));
            let pattern = Test::new(Matrix4::from_translation(Vector3::new(0.5, 1., 1.5)));
            assert_eq!(
                pattern.at_shape(object, Point3::new(2.5, 3., 3.5)),
                RGB::new(0.75, 0.5, 0.25)
            );
        }
    }
}
