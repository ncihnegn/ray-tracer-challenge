use cgmath::{InnerSpace, Point3};
use ray_tracer_challenge::{
    canvas::Canvas,
    intersection::hit,
    light::Light,
    pattern::Pattern,
    ray::Ray,
    shape::{sphere::Sphere, Shape},
};
use rgb::RGB;
use std::fs;

fn main() {
    let ray_origin = Point3::new(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;
    let half = wall_size / 2.;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut sphere = Sphere::default();
    sphere.material.pattern = Pattern::Solid(RGB::new(1., 0.2, 1.));
    let shape = Shape::Sphere(sphere);
    let light = Light::new(Point3::new(-10., 10., -10.), RGB::new(1., 1., 1.));
    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels - 1 {
            // spans from -half to half
            let world_x = -half + pixel_size * x as f32;
            let target = Point3::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, target - ray_origin);
            let xs = shape.intersect(r);
            if let Some(hit) = hit(&xs) {
                let point = r.position(hit.t);
                let object = hit.object;
                let normal = object.normal_at(point, hit.uv).unwrap();
                let eye = -r.direction.normalize();
                canvas.pixels[y][x] = object
                    .material()
                    .unwrap()
                    .lighting(light, point, eye, normal, false);
            }
        }
    }
    let _ = fs::create_dir("output");
    fs::write("output/sphere.ppm", canvas.to_ppm()).expect("Unable to write file");
}
