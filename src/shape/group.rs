use crate::{intersection::Intersection, material::Material, ray::Ray, shape::Shape};
use cgmath::{
    abs_diff_eq, BaseFloat, EuclideanSpace, InnerSpace, Matrix, Matrix4, Point3, SquareMatrix,
    Vector3,
};
use derivative::Derivative;
use derive_more::Constructor;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

#[derive(Clone, Constructor, Debug, Derivative)]
#[derivative(PartialEq)]
pub struct ShapeWrapper<T> {
    pub shape: Shape<T>,
    #[derivative(PartialEq = "ignore")]
    pub parent: Option<Weak<RefCell<ShapeWrapper<T>>>>,
}

impl<T: BaseFloat> ShapeWrapper<T> {
    fn world_to_object(&self, point: Point3<T>) -> Option<Point3<T>> {
        let pp = Some(point);
        let o = self.parent.as_ref().map_or(pp, |weak| {
            weak.upgrade()
                .map_or(pp, |rc| rc.borrow().world_to_object(point))
        });
        self.shape
            .transform()
            .invert()
            .and_then(|i| o.map(|p| Point3::from_vec((i * p.to_homogeneous()).truncate())))
    }

    fn normal_to_world(&self, normal: Vector3<T>) -> Option<Vector3<T>> {
        let ov = self.shape.transform().invert().map(|i| {
            (i.transpose() * normal.extend(T::zero()))
                .truncate()
                .normalize()
        });
        self.parent.as_ref().map_or(ov, |weak| {
            weak.upgrade()
                .map_or(ov, |rc| ov.and_then(|v| rc.borrow().normal_to_world(v)))
        })
    }

    pub fn normal_at(&self, world_point: Point3<T>) -> Option<Vector3<T>> {
        self.world_to_object(world_point)
            .map(|local_point| self.shape.local_normal_at(local_point))
            .map(|local_normal| self.normal_to_world(local_normal).unwrap())
    }
}

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

    pub fn local_intersect(&self, ray: Ray<T>) -> Vec<Intersection<T>> {
        let mut xs = self
            .children
            .iter()
            .flat_map(|r| r.borrow().shape.intersect(ray))
            .collect::<Vec<_>>();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn local_normal_at(&self, point: Point3<T>) -> Vector3<T> {
        Vector3::unit_x()
    }
}

mod tests {
    use crate::shape::sphere::Sphere;

    use super::*;
    use cgmath::{assert_relative_eq, Rad, Zero};
    use std::f32::consts::{FRAC_PI_2, PI, SQRT_2};

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

            assert_eq!(rc.borrow().shape.as_group().unwrap().children.len(), 3);
            let xs = rc
                .borrow()
                .shape
                .as_group()
                .unwrap()
                .local_intersect(Ray::new(Point3::new(0., 0., -5.), Vector3::unit_z()));
            assert_eq!(xs.len(), 4);
            assert_eq!(
                xs[0].object,
                rc.borrow().shape.as_group().unwrap().children[1]
                    .borrow()
                    .shape
            );
            assert_eq!(
                xs[1].object,
                rc.borrow().shape.as_group().unwrap().children[1]
                    .borrow()
                    .shape
            );
            assert_eq!(
                xs[2].object,
                rc.borrow().shape.as_group().unwrap().children[0]
                    .borrow()
                    .shape
            );
            assert_eq!(
                xs[3].object,
                rc.borrow().shape.as_group().unwrap().children[0]
                    .borrow()
                    .shape
            );
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
        push(&r1.borrow().shape.as_group().unwrap().children[0], shape);
        assert_relative_eq!(
            r1.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .shape
                .as_group()
                .unwrap()
                .children[0]
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
        push(&r1.borrow().shape.as_group().unwrap().children[0], shape);
        let frac_1_sqrt_3 = 3.0_f32.sqrt().recip();
        assert_relative_eq!(
            r1.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .shape
                .as_group()
                .unwrap()
                .children[0]
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
        push(&r1.borrow().shape.as_group().unwrap().children[0], shape);
        assert_relative_eq!(
            r1.borrow().shape.as_group().unwrap().children[0]
                .borrow()
                .shape
                .as_group()
                .unwrap()
                .children[0]
                .borrow()
                .normal_at(Point3::new(1.7321, 1.1547, -5.5774))
                .unwrap(),
            Vector3::new(0.2857, 0.4286, -0.8571),
            max_relative = 0.001
        );
    }
}
