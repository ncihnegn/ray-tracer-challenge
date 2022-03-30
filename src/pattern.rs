use crate::{material::Material, shape::Shape};
use cgmath::{BaseFloat, EuclideanSpace, Matrix4, Point3, SquareMatrix};
use derive_more::Constructor;
use num_traits::cast;
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

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Gradient<T> {
    a: RGB<T>,
    b: RGB<T>,
    transform: Matrix4<T>,
}

impl<T: BaseFloat> TraitPattern<T> for Gradient<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn at(&self, point: Point3<T>) -> RGB<T> {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();
        self.a + distance * fraction
    }
}

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Ring<T> {
    a: RGB<T>,
    b: RGB<T>,
    transform: Matrix4<T>,
}

impl<T: BaseFloat> TraitPattern<T> for Ring<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn at(&self, point: Point3<T>) -> RGB<T> {
        let i: i32 = cast((point.x * point.x + point.z * point.z).sqrt().floor()).unwrap();
        if i % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Checker<T> {
    a: RGB<T>,
    b: RGB<T>,
    transform: Matrix4<T>,
}

impl<T: BaseFloat> TraitPattern<T> for Checker<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn at(&self, point: Point3<T>) -> RGB<T> {
        let i: i32 = cast(point.x.floor() + point.y.floor() + point.z.floor()).unwrap();
        if i % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Clone, Constructor, Copy, Debug, PartialEq)]
pub struct Test<T> {
    transform: Matrix4<T>,
}

impl<T: BaseFloat> TraitPattern<T> for Test<T> {
    fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    fn at(&self, point: Point3<T>) -> RGB<T> {
        RGB::new(point.x, point.y, point.z)
    }
}

mod tests {
    use crate::shape::Sphere;

    use super::*;
    use cgmath::Vector3;

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        {
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
        {
            let gradient = Gradient::new(white, black, Matrix4::identity());
            assert_eq!(gradient.at(Point3::origin()), white);
            assert_eq!(
                gradient.at(Point3::new(0.25, 0., 0.)),
                RGB::new(0.75, 0.75, 0.75)
            );
            assert_eq!(
                gradient.at(Point3::new(0.5, 0., 0.)),
                RGB::new(0.5, 0.5, 0.5)
            );
            assert_eq!(
                gradient.at(Point3::new(0.75, 0., 0.)),
                RGB::new(0.25, 0.25, 0.25)
            );
        }
        {
            let ring = Ring::new(white, black, Matrix4::identity());
            assert_eq!(ring.at(Point3::origin()), white);
            assert_eq!(ring.at(Point3::from_vec(Vector3::unit_x())), black);
            assert_eq!(ring.at(Point3::from_vec(Vector3::unit_z())), black);
            assert_eq!(ring.at(Point3::new(0.708, 0., 0.708)), black);
        }
        {
            let checker = Checker::new(white, black, Matrix4::identity());
            assert_eq!(checker.at(Point3::origin()), white);
            assert_eq!(checker.at(Point3::new(0.99, 0., 0.)), white);
            assert_eq!(checker.at(Point3::new(1.01, 0., 0.)), black);
            assert_eq!(checker.at(Point3::new(0., 0.99, 0.)), white);
            assert_eq!(checker.at(Point3::new(0., 1.01, 0.)), black);
            assert_eq!(checker.at(Point3::new(0., 0., 0.99)), white);
            assert_eq!(checker.at(Point3::new(0., 0., 1.01)), black);
        }
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
