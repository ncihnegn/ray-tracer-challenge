use crate::pattern::TraitPattern;
use cgmath::{BaseFloat, Matrix4, Point3};
use derive_more::Constructor;
use rgb::RGB;

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
    use super::*;
    use cgmath::{EuclideanSpace, SquareMatrix};

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
        let test = Test::new(Matrix4::identity());
        assert_eq!(test.at(Point3::origin()), black);
        assert_eq!(test.at(Point3::new(1., 1., 1.)), white);
    }
}
