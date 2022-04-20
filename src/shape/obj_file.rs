use crate::shape::{group::push, Group, Shape, ShapeWrapper, Triangle};
use cgmath::{BaseFloat, Point3};
use std::{cell::RefCell, rc::Rc, str::FromStr};

pub struct Parser<T> {
    default_group: Rc<RefCell<ShapeWrapper<T>>>,
    vertices: Vec<Point3<T>>,
}

impl<T: BaseFloat + FromStr + Default> Parser<T> {
    pub fn parse_obj_file(s: &str) -> Parser<T> {
        let mut vertices = Vec::new();
        let mut group = Group::default();
        let default_group = Rc::new(RefCell::new(ShapeWrapper::new(Shape::Group(group), None)));
        for l in s.lines() {
            let mut iter = l.split_whitespace();
            match iter.next() {
                Some("f") => push(
                    &default_group,
                    Shape::Triangle(Triangle::from(
                        vertices[usize::from_str(iter.next().unwrap()).unwrap() - 1],
                        vertices[usize::from_str(iter.next().unwrap()).unwrap() - 1],
                        vertices[usize::from_str(iter.next().unwrap()).unwrap() - 1],
                    )),
                ),
                Some("l") => {}
                Some("v") => vertices.push(Point3::new(
                    T::from_str(iter.next().unwrap()).unwrap_or(T::zero()),
                    T::from_str(iter.next().unwrap()).unwrap_or(T::zero()),
                    T::from_str(iter.next().unwrap()).unwrap_or(T::zero()),
                )),
                Some("vn") => {}
                Some("vp") => {}
                Some("vt") => {}
                _ => {}
            }
        }
        Parser {
            default_group,
            vertices,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_obj_file() {
        {
            let parser = Parser::<f32>::parse_obj_file(
                r#"
There was a young lady named Bright
who traveled much faster than light.
She set out one day
in a relative way,
and came back the previous night.
"#,
            );
            assert_eq!(parser.vertices, vec![]);
        }
        {
            let parser = Parser::<f32>::parse_obj_file(
                r#"
v -1 1 0
v -1.0000 0.5000 0.0000
v 1 0 0
v 1 1 0
"#,
            );
            assert_eq!(
                parser.vertices,
                vec![
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0.5, 0.),
                    Point3::new(1., 0., 0.),
                    Point3::new(1., 1., 0.),
                ]
            );
        }
        {
            let parser = Parser::<f32>::parse_obj_file(
                r#"
v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

f 1 2 3
f 1 3 4
"#,
            );
            let children = parser
                .default_group
                .borrow()
                .shape
                .as_group()
                .unwrap()
                .children
                .clone();
            assert_eq!(
                children[0].borrow().shape,
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                ))
            );
            assert_eq!(
                children[1].borrow().shape,
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(1., 0., 0.),
                    Point3::new(1., 1., 0.),
                ))
            );
        }
    }
}
