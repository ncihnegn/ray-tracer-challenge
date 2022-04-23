use crate::shape::{get_link, group::push, Group, Shape, ShapeLink, ShapeWrapper, Triangle};
use cgmath::{BaseFloat, Point3};
use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

pub struct Parser<T> {
    groups: HashMap<String, ShapeLink<T>>,
    vertices: Vec<Point3<T>>,
}

fn fan_tranigulation<T: BaseFloat + Default>(
    vertices: &Vec<Point3<T>>,
    index: &Vec<usize>,
) -> Vec<Triangle<T>> {
    // Assuming a convex polygon
    (1..index.len() - 1)
        .map(|i| index[i])
        .map(|i| Triangle::from(vertices[index[0]], vertices[i], vertices[i + 1]))
        .collect::<Vec<_>>()
}

impl<T: BaseFloat + FromStr + Default> Parser<T> {
    pub fn parse_obj_file(s: &str) -> Parser<T> {
        let mut vertices = Vec::new();
        let mut group = Group::default();
        let mut groups = HashMap::new();
        groups.insert("default".to_string(), get_link(Shape::Group(group)));
        let mut current_label = "default";
        for l in s.lines() {
            let mut iter = l.split_whitespace();
            match iter.next() {
                Some("f") => {
                    let index = iter
                        .map(|s| usize::from_str(s).unwrap() - 1)
                        .collect::<Vec<_>>();
                    for tri in fan_tranigulation(&vertices, &index) {
                        push(&groups.get(current_label).unwrap(), Shape::Triangle(tri));
                    }
                }
                Some("g") => {
                    if let Some(label) = iter.next() {
                        let mut group = Group::default();
                        groups.insert(label.to_string(), get_link(Shape::Group(group)));
                        current_label = label;
                    }
                }
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
        Parser { groups, vertices }
    }

    pub fn obj_to_group(self) -> ShapeLink<T> {
        let top_group = get_link(Shape::Group(Group::default()));
        for (_, group) in self.groups {
            let shape = group.borrow().shape.clone();
            if !shape.as_group().unwrap().children.is_empty() {
                push(&top_group, shape);
            }
        }
        top_group
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
                .groups
                .get("default")
                .unwrap()
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
        {
            let parser = Parser::<f32>::parse_obj_file(
                r#"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0
                v 0 2 0

                f 1 2 3 4 5
                "#,
            );
            let children = parser
                .groups
                .get("default")
                .unwrap()
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
            assert_eq!(
                children[2].borrow().shape,
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(1., 1., 0.),
                    Point3::new(0., 2., 0.),
                ))
            );
        }
        {
            let parser = Parser::<f32>::parse_obj_file(
                r#"
                v -1 1 0
                v -1 0 0
                v 1 0 0
                v 1 1 0

                g FirstGroup
                f 1 2 3
                g SecondGroup
                f 1 3 4
                "#,
            );
            assert_eq!(
                parser
                    .groups
                    .get("FirstGroup")
                    .unwrap()
                    .borrow()
                    .shape
                    .as_group()
                    .unwrap()
                    .children[0]
                    .borrow()
                    .shape,
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                ))
            );
            assert_eq!(
                parser
                    .groups
                    .get("SecondGroup")
                    .unwrap()
                    .borrow()
                    .shape
                    .as_group()
                    .unwrap()
                    .children[0]
                    .borrow()
                    .shape,
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(1., 0., 0.),
                    Point3::new(1., 1., 0.),
                ))
            );
        }
    }
}
