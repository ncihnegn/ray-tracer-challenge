use crate::{bounds::Bounds, intersection::Intersection, ray::Ray, shape::ShapeLink};
use cgmath::{BaseFloat, Matrix4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operation {
    Union,
    Intersect,
    Difference,
}

#[derive(Clone, derive_more::Constructor, Debug, PartialEq)]
pub struct ConstructiveSolidGeometry<T> {
    pub transform: Matrix4<T>,
    pub op: Operation,
    pub left: ShapeLink<T>,
    pub right: ShapeLink<T>,
}

fn intersection_allowed(op: Operation, lhit: bool, inl: bool, inr: bool) -> bool {
    match op {
        Operation::Union => (lhit & !inr) | (!lhit & !inl),
        Operation::Intersect => (lhit & inr) | (!lhit & inl),
        Operation::Difference => (lhit & !inr) | (!lhit & inl),
    }
}

impl<T: BaseFloat> ConstructiveSolidGeometry<T> {
    pub fn bounds(&self) -> Bounds<T> {
        todo!();
    }

    fn filter_intersections(&self, xs: &[Intersection<T>]) -> Vec<Intersection<T>> {
        let mut inl = false;
        let mut inr = false;
        let mut result = Vec::new();
        for i in xs {
            let lhit = self.left.borrow().shape.include(&i.object);
            if intersection_allowed(self.op, lhit, inl, inr) {
                result.push(i.clone());
            }
            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }
        result
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let mut v = self.left.borrow().shape.intersect(ray);
        v.append(&mut self.right.borrow().shape.intersect(ray));
        v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Less));
        self.filter_intersections(&v)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::SquareMatrix;

    use super::*;
    use crate::shape::{get_link, Cube, Shape, Sphere};
    use cgmath::{Point3, Vector3};

    fn filter_intersections() {
        let sphere = Shape::Sphere(Sphere::default());
        let cube = Shape::Cube(Cube::default());
        for (op, x0, x1) in [
            (Operation::Union, 0, 3),
            (Operation::Intersect, 1, 2),
            (Operation::Difference, 0, 1),
        ] {
            let c = ConstructiveSolidGeometry::new(
                Matrix4::identity(),
                op,
                get_link(sphere.clone()),
                get_link(cube.clone()),
            );
            let xs = vec![
                Intersection::new(1., sphere.clone(), None),
                Intersection::new(2., cube.clone(), None),
                Intersection::new(3., sphere.clone(), None),
                Intersection::new(4., cube.clone(), None),
            ];
            assert_eq!(
                c.filter_intersections(&xs),
                vec![xs[x0].clone(), xs[x1].clone()]
            );
        }
    }

    fn local_intersect() {
        {
            let sphere = Shape::Sphere(Sphere::default());
            let cube = Shape::Cube(Cube::default());
            let c = ConstructiveSolidGeometry::new(
                Matrix4::identity(),
                Operation::Union,
                get_link(sphere.clone()),
                get_link(cube.clone()),
            );
            let ray = Ray::new(Point3::new(0., 2., -5.), Vector3::unit_z());
            assert_eq!(c.local_intersect(ray), vec![]);
        }
        {
            let s1 = Shape::Sphere(Sphere::default());
            let mut s = Sphere::default();
            s.transform = Matrix4::from_translation(Vector3::unit_z() * 0.5);
            let s2 = Shape::Sphere(s);

            let c = ConstructiveSolidGeometry::new(
                Matrix4::identity(),
                Operation::Union,
                get_link(s1.clone()),
                get_link(s2.clone()),
            );
            let ray = Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z());
            assert_eq!(
                c.local_intersect(ray),
                vec![
                    Intersection::new(4., s1, None),
                    Intersection::new(2., s2, None)
                ]
            );
        }
    }
}
