use cgmath::BaseFloat;
use cgmath::Matrix4;

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<T> {
    pub transform: Matrix4<T>,
    pub t: T,
}

impl<T: BaseFloat> Intersection<T> {
    pub fn new(transform: Matrix4<T>, t: T) -> Intersection<T> {
        Intersection::<T> { transform, t }
    }
}

pub fn hit<T: BaseFloat>(v: Vec<Intersection<T>>) -> Option<Intersection<T>> {
    v.iter()
        .filter(|i| i.t >= T::zero())
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .cloned()
}

mod tests {
    use super::*;

    #[test]
    fn hit() {
        let sphere = Matrix4::from_scale(1.);
        // All have positive t
        let i1 = Intersection::new(sphere, 1.);
        assert_eq!(
            super::hit(vec![i1.clone(), Intersection::new(sphere, 2.)]),
            Some(i1.clone())
        );
        // Some have negative t
        assert_eq!(
            super::hit(vec![Intersection::new(sphere, -1.), i1.clone()]),
            Some(i1)
        );
        // All have negative t
        assert_eq!(
            super::hit(vec![
                Intersection::new(sphere, -2.),
                Intersection::new(sphere, -1.)
            ]),
            None
        );
    }
}
