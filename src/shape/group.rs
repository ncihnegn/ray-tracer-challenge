use crate::{
    bounds::Bounds,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    shape::{Shape, ShapeWrapper},
};
use cgmath::{
    abs_diff_eq, BaseFloat, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix,
    Vector3,
};
use derive_more::Constructor;
use std::{cell::RefCell, cmp::Ordering::Less, fmt::Debug, rc::Rc};

type ShapeLink<T> = Rc<RefCell<ShapeWrapper<T>>>;

#[derive(Clone, Constructor, Debug, PartialEq)]
pub struct Group<T> {
    pub transform: Matrix4<T>,
    pub children: Vec<ShapeLink<T>>,
}

pub fn push<T>(parent: &Rc<RefCell<ShapeWrapper<T>>>, shape: Shape<T>) {
    let child = Rc::new(RefCell::new(ShapeWrapper::new(
        shape,
        Some(Rc::downgrade(parent)),
    )));
    parent
        .borrow_mut()
        .shape
        .as_group_mut()
        .unwrap()
        .children
        .push(child);
}

impl<T: BaseFloat + Default> Default for Group<T> {
    fn default() -> Group<T> {
        Group::<T> {
            transform: Matrix4::identity(),
            children: Vec::new(),
        }
    }
}

impl<T: BaseFloat + Debug> Group<T> {
    pub fn transform(&self) -> Matrix4<T> {
        self.transform
    }

    pub fn material(&self) -> Option<Material<T>> {
        None
    }
    pub fn bounds(&self) -> Option<Bounds<T>> {
        Bounds::from_all_points(
            &self
                .children
                .iter()
                .filter_map(|rc| {
                    let shape = &rc.borrow().shape;
                    shape.bounds().map(|b| b.transform(shape.transform()))
                })
                .flatten()
                .collect::<Vec<_>>(),
        )
    }

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        if self.bounds().map_or(true, |b| b.is_intersected_with(ray)) {
            let mut xs = self
                .children
                .iter()
                .flat_map(|rc| rc.borrow().shape.intersect(ray))
                .collect::<Vec<_>>();
            xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Less));
            xs
        } else {
            vec![]
        }
    }
}

mod tests {
    use super::*;
    use crate::shape::sphere::Sphere;
    use cgmath::{assert_relative_eq, Rad, Zero};
    use std::f32::consts::{FRAC_PI_2, PI, SQRT_2};

    #[test]
    fn bounds() {
        let shape = Shape::Group(Group::<f32>::new(Matrix4::from_scale(2.), Vec::new()));
        let rc = Rc::new(RefCell::new(ShapeWrapper::new(shape, None)));
        push(
            &rc,
            Shape::Sphere(Sphere::new(
                Matrix4::from_translation(Vector3::unit_x() * 5.),
                Material::default(),
            )),
        );
    }

    #[test]
    fn local_intersect() {
        {
            let group = Group::<f32>::default();
            assert_eq!(
                group.local_intersect(Ray::new(Point3::origin(), Vector3::unit_z())),
                vec![]
            );
        }
        {
            let rc = Rc::new(RefCell::new(ShapeWrapper::new(
                Shape::Group(Group::<f32>::default()),
                None,
            )));
            push(&rc, Shape::Sphere(Sphere::default()));
            push(
                &rc,
                Shape::Sphere(Sphere::new(
                    Matrix4::from_translation(Vector3::unit_z() * -3.),
                    Material::default(),
                )),
            );
            push(
                &rc,
                Shape::Sphere(Sphere::new(
                    Matrix4::from_translation(Vector3::unit_x() * 5.),
                    Material::default(),
                )),
            );
            let group = rc.borrow().shape.as_group().unwrap().clone();
            assert_eq!(group.children.len(), 3);
            let xs = group.local_intersect(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()));
            assert_eq!(xs.len(), 4);
            assert_eq!(xs[0].object, group.children[1].borrow().shape);
            assert_eq!(xs[1].object, group.children[1].borrow().shape);
            assert_eq!(xs[2].object, group.children[0].borrow().shape);
            assert_eq!(xs[3].object, group.children[0].borrow().shape);
        }
        {
            let shape = Shape::Group(Group::<f32>::new(Matrix4::from_scale(2.), Vec::new()));
            let rc = Rc::new(RefCell::new(ShapeWrapper::new(shape, None)));
            push(
                &rc,
                Shape::Sphere(Sphere::new(
                    Matrix4::from_translation(Vector3::unit_x() * 5.),
                    Material::default(),
                )),
            );
            assert_eq!(
                rc.borrow()
                    .shape
                    .intersect(Ray::new(Point3::new(10., 0., -10.), Vector3::unit_z()))
                    .len(),
                2
            );
        }
    }

    #[test]
    fn world_to_object() {
        let g1 = ShapeWrapper::new(
            Shape::Group(Group::new(
                Matrix4::from_angle_y(Rad(FRAC_PI_2)),
                Vec::new(),
            )),
            None,
        );
        let g2 = Shape::Group(Group::new(Matrix4::from_scale(2.), Vec::new()));
        let r1 = Rc::new(RefCell::new(g1));
        push(&r1, g2);
        let shape = Shape::Sphere(Sphere::new(
            Matrix4::from_translation(Vector3::unit_x() * 5.),
            Material::default(),
        ));
        let child = r1.borrow().shape.as_group().unwrap().children[0].clone();
        push(&child, shape);
        assert_relative_eq!(
            child.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .world_to_object(Point3::new(-2., 0., -10.))
                .unwrap(),
            Point3::new(0., 0., -1.),
            max_relative = 0.00001
        );
    }

    #[test]
    fn normal_to_world() {
        let g1 = ShapeWrapper::new(
            Shape::Group(Group::new(
                Matrix4::from_angle_y(Rad(FRAC_PI_2)),
                Vec::new(),
            )),
            None,
        );
        let g2 = Shape::Group(Group::new(
            Matrix4::from_nonuniform_scale(1., 2., 3.),
            Vec::new(),
        ));
        let r1 = Rc::new(RefCell::new(g1));
        push(&r1, g2);
        let shape = Shape::Sphere(Sphere::new(
            Matrix4::from_translation(Vector3::unit_x() * 5.),
            Material::default(),
        ));
        let child = r1.borrow().shape.as_group().unwrap().children[0].clone();
        push(&child, shape);
        let frac_1_sqrt_3 = 3.0_f32.sqrt().recip();
        assert_relative_eq!(
            child.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .normal_to_world(Vector3::new(frac_1_sqrt_3, frac_1_sqrt_3, frac_1_sqrt_3))
                .unwrap(),
            Vector3::new(0.2857, 0.4286, -0.8571),
            max_relative = 0.0001
        );
    }

    #[test]
    fn normal_at() {
        let g1 = ShapeWrapper::new(
            Shape::Group(Group::new(
                Matrix4::from_angle_y(Rad(FRAC_PI_2)),
                Vec::new(),
            )),
            None,
        );
        let g2 = Shape::Group(Group::new(
            Matrix4::from_nonuniform_scale(1., 2., 3.),
            Vec::new(),
        ));
        let r1 = Rc::new(RefCell::new(g1));
        push(&r1, g2);
        let shape = Shape::Sphere(Sphere::new(
            Matrix4::from_translation(Vector3::unit_x() * 5.),
            Material::default(),
        ));
        let child = r1.borrow().shape.as_group().unwrap().children[0].clone();
        push(&child, shape);
        assert_relative_eq!(
            child.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .normal_at(Point3::new(1.7321, 1.1547, -5.5774))
                .unwrap(),
            Vector3::new(0.2857, 0.4286, -0.8571),
            max_relative = 0.001
        );
    }
}
