use crate::pattern::TraitPattern;
use cgmath::{BaseFloat, Matrix4, Point3};
use rgb::RGB;

#[derive(Clone, derive_more::Constructor, Copy, Debug, PartialEq)]
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

mod tests {
    use super::*;
    use cgmath::{EuclideanSpace, SquareMatrix};

    #[test]
    fn at() {
        let white = RGB::new(1., 1., 1.);
        let black = RGB::new(0., 0., 0.);
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
}
