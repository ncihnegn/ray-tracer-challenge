use crate::{
    material::Material,
    shape::{
        get_rc,
        group::{push, push_link},
        Group, Shape, ShapeRc, SmoothTriangle, Triangle,
    },
};
use cgmath::{BaseFloat, Point3, Vector3};
use std::{collections::HashMap, rc::Rc, str::FromStr};

pub struct Parser<T> {
    groups: HashMap<String, ShapeRc<T>>,
    vertices: Vec<Point3<T>>,
    normals: Vec<Vector3<T>>,
}

fn fan_tranigulation<T: BaseFloat + Default>(
    vertices: &[Point3<T>],
    normals: &[Vector3<T>],
    index: &[Vec<Option<usize>>],
) -> Vec<Shape<T>> {
    // Assuming a convex polygon
    (1..index.len() - 1)
        .map(|i| &index[i])
        .map(|v| {
            if v.len() == 1 {
                Shape::Triangle(Triangle::from(
                    vertices[index[0][0].unwrap() - 1],
                    vertices[v[0].unwrap() - 1],
                    vertices[v[0].unwrap()],
                ))
            } else {
                Shape::SmoothTriangle(SmoothTriangle::new(
                    Material::default(),
                    vertices[index[0][0].unwrap() - 1],
                    vertices[v[0].unwrap() - 1],
                    vertices[v[0].unwrap()],
                    normals[index[0][2].unwrap() - 1],
                    normals[v[2].unwrap() - 1],
                    normals[v[2].unwrap()],
                    None,
                ))
            }
        })
        .collect::<Vec<_>>()
}

impl<T: BaseFloat + FromStr + Default> Parser<T> {
    pub fn parse_obj_file(s: &str) -> Parser<T> {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let group = Group::default();
        let mut groups = HashMap::new();
        groups.insert("default".to_string(), get_rc(Shape::Group(group)));
        let mut current_label = "default";
        for l in s.lines() {
            let mut iter = l.split_whitespace();
            match iter.next() {
                Some("f") => {
                    let index = iter
                        .map(|s| {
                            s.split_terminator('/')
                                .map(|s| usize::from_str(s).ok())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>();
                    for tri in fan_tranigulation(&vertices, &normals, &index) {
                        push(groups.get(current_label).unwrap(), tri);
                    }
                }
                Some("g") => {
                    if let Some(label) = iter.next() {
                        let group = Group::default();
                        groups.insert(label.to_string(), get_rc(Shape::Group(group)));
                        current_label = label;
                    }
                }
                Some("l") => {}
                Some("v") => vertices.push(Point3::new(
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                )),
                Some("vn") => normals.push(Vector3::new(
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                    T::from_str(iter.next().unwrap()).unwrap_or_default(),
                )),
                Some("vp") => {}
                Some("vt") => {}
                _ => {}
            }
        }
        Parser {
            groups,
            vertices,
            normals,
        }
    }

    pub fn obj_to_group(self) -> Shape<T> {
        let top_group = get_rc(Shape::Group(Group::default()));
        for (_, group) in self.groups {
            if !group.borrow().as_group().unwrap().children.is_empty() {
                push_link(&top_group, group);
            }
        }
        Rc::try_unwrap(top_group).unwrap().into_inner()
    }
}

mod tests {
    use super::*;
    use std::ops::Deref;

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
                .as_group()
                .unwrap()
                .children
                .clone();
            assert_eq!(
                *children[0].borrow().deref(),
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                ))
            );
            assert_eq!(
                *children[1].borrow().deref(),
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
                .as_group()
                .unwrap()
                .children
                .clone();
            assert_eq!(
                *children[0].borrow().deref(),
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                ))
            );
            assert_eq!(
                *children[1].borrow().deref(),
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(1., 0., 0.),
                    Point3::new(1., 1., 0.),
                ))
            );
            assert_eq!(
                *children[2].borrow().deref(),
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
                *parser
                    .groups
                    .get("FirstGroup")
                    .unwrap()
                    .borrow()
                    .as_group()
                    .unwrap()
                    .children[0]
                    .borrow()
                    .deref(),
                Shape::Triangle(Triangle::from(
                    Point3::new(-1., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                ))
            );
            assert_eq!(
                *parser
                    .groups
                    .get("SecondGroup")
                    .unwrap()
                    .borrow()
                    .as_group()
                    .unwrap()
                    .children[0]
                    .borrow()
                    .deref(),
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
                v 0 1 0
                v -1 0 0
                v 1 0 0

                vn -1 0 0
                vn 1 0 0
                vn 0 1 0

                f 1//3 2//1 3//2
                f 1/0/3 2/102/1 3/14/2
                "#,
            );
            let children = parser
                .groups
                .get("default")
                .unwrap()
                .borrow()
                .as_group()
                .unwrap()
                .children
                .clone();
            assert_eq!(
                *children[0].borrow().deref(),
                Shape::SmoothTriangle(SmoothTriangle::new(
                    Material::default(),
                    Point3::new(0., 1., 0.),
                    Point3::new(-1., 0., 0.),
                    Point3::new(1., 0., 0.),
                    Vector3::unit_y(),
                    -Vector3::unit_x(),
                    Vector3::unit_x(),
                    None,
                ))
            );
        }
    }
}
