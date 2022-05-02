// Using f32 will bring acnes while over_point is not necessary.
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use cgmath::{Matrix4, Point3, Rad, Vector3};
use ray_tracer_challenge::{
    camera::Camera,
    light::Light,
    material::Material,
    pattern::Pattern,
    shape::{sphere::Sphere, Shape},
    world::World,
};
use rgb::RGB;
use std::fs;

fn main() {
    let mut room_material = Material::default();
    room_material.pattern = Pattern::Solid(RGB::new(1., 0.9, 0.9));
    room_material.specular = 0.;

    let mut floor = Sphere::default();
    floor.transform = Matrix4::from_nonuniform_scale(10., 0.01, 10.);
    floor.material = room_material;

    let mut left_wall = Sphere::default();
    left_wall.transform = Matrix4::from_translation(Vector3::unit_z() * 5.)
        * Matrix4::from_angle_y(Rad(-FRAC_PI_4))
        * Matrix4::from_angle_x(Rad(FRAC_PI_2))
        * floor.transform;
    left_wall.material = room_material;

    let mut right_wall = Sphere::default();
    right_wall.transform = Matrix4::from_translation(Vector3::unit_z() * 5.)
        * Matrix4::from_angle_y(Rad(FRAC_PI_4))
        * Matrix4::from_angle_x(Rad(FRAC_PI_2))
        * floor.transform;
    right_wall.material = room_material;

    let mut sphere_material = Material::default();
    sphere_material.diffuse = 0.7;
    sphere_material.specular = 0.3;

    let mut middle = Sphere::default();
    middle.transform = Matrix4::from_translation(Vector3::new(-0.5, 1., 0.5));
    middle.material = sphere_material;
    middle.material.pattern = Pattern::Solid(RGB::new(0.1, 1., 0.5));

    let mut right = Sphere::default();
    right.transform =
        Matrix4::from_translation(Vector3::new(1.5, 0.5, -0.5)) * Matrix4::from_scale(0.5);
    right.material = sphere_material;
    right.material.pattern = Pattern::Solid(RGB::new(0.5, 1., 0.1));

    let mut left = Sphere::default();
    left.transform =
        Matrix4::from_translation(Vector3::new(-1.5, 0.33, -0.75)) * Matrix4::from_scale(0.33);
    left.material = sphere_material;
    left.material.pattern = Pattern::Solid(RGB::new(1., 0.8, 0.1));

    let light = Light::new(Point3::new(-10., 10., -10.), RGB::new(1., 1., 1.));
    let mut camera = Camera::from(400, 200, FRAC_PI_3);
    camera.transform = Matrix4::look_at_rh(
        Point3::new(0., 1.5, -5.),
        Point3::new(0., 1., 0.),
        Vector3::unit_y(),
    );

    let world = World::new(
        light,
        vec![
            Shape::Sphere(floor),
            Shape::Sphere(left_wall),
            Shape::Sphere(right_wall),
            Shape::Sphere(left),
            Shape::Sphere(middle),
            Shape::Sphere(right),
        ],
        5,
    );

    let canvas = camera.render(world);
    let _ = fs::create_dir("output");
    fs::write("output/spheres.ppm", canvas.to_ppm()).expect("Unable to write file");
}
