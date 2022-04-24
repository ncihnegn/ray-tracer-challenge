use crate::pattern::TraitPattern;
use cgmath::{BaseFloat, Matrix4, Point3};
use derive_more::Constructor;
use num_traits::cast;
use rgb::RGB;

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

mod tests {
    use super::*;
    use cgmath::{EuclideanSpace, SquareMatrix};

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        let checker = Checker::new(white, black, Matrix4::identity());
        assert_eq!(checker.at(Point3::origin()), white);
        assert_eq!(checker.at(Point3::new(0.99, 0., 0.)), white);
        assert_eq!(checker.at(Point3::new(1.01, 0., 0.)), black);
        assert_eq!(checker.at(Point3::new(0., 0.99, 0.)), white);
        assert_eq!(checker.at(Point3::new(0., 1.01, 0.)), black);
        assert_eq!(checker.at(Point3::new(0., 0., 0.99)), white);
        assert_eq!(checker.at(Point3::new(0., 0., 1.01)), black);
    }
}
