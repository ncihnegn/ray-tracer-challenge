use cgmath::{BaseFloat, InnerSpace, Point3, Vector3};
use ray_tracer_challenge::canvas::Canvas;
use rgb::RGB;
use std::fs;

#[derive(Clone, Copy)]
struct Projectile<T> {
    position: Point3<T>,
    velocity: Vector3<T>,
}

#[derive(Clone, Copy)]
struct Environment<T> {
    gravity: Vector3<T>,
    wind: Vector3<T>,
}

fn tick<T: BaseFloat>(env: Environment<T>, proj: Projectile<T>) -> Projectile<T> {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile::<T> { position, velocity }
}

fn main() {
    let start = Point3::new(0., 1., 0.);
    let velocity = Vector3::new(1., 1.8, 0.).normalize() * 11.25;
    let mut proj = Projectile {
        position: start,
        velocity,
    };
    let gravity = -0.1 * Vector3::unit_y();
    let wind = -0.01 * Vector3::unit_x();
    let environment = Environment { gravity, wind };
    let mut canvas = Canvas::new(900, 550);

    while proj.position.y >= 0.0 && proj.position.x >= 0.0 {
        canvas.pixels[canvas.height - (proj.position.y as usize)][proj.position.x as usize] =
            RGB::new(1., 0., 0.);
        proj = tick(environment, proj);
    }
    let _ = fs::create_dir("output");
    fs::write("output/projectile.ppm", canvas.to_ppm()).expect("Unable to write file");
}
