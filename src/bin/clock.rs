use cgmath::{Matrix3, Rad, Vector3};
use ray_tracer_challenge::canvas::Canvas;
use rgb::RGB;
use std::{f32::consts::FRAC_PI_6, fs};

fn main() {
    let mut canvas = Canvas::new(240, 240);
    let radius = (canvas.width * 3 / 8) as f32;

    let twelve = Vector3::new(0., 0., 1.);
    let rotate_one_hour = Matrix3::from_angle_y(Rad(FRAC_PI_6));

    let mut hour = twelve;
    for _ in 1..=12 {
        let y = (canvas.height / 2) as isize - (hour.z * radius) as isize;
        let x = (canvas.width / 2) as isize + (hour.x * radius) as isize;
        canvas.pixels[y as usize][x as usize] = RGB::new(1., 1., 1.);
        hour = rotate_one_hour * hour;
    }
    let _ = fs::create_dir("output");
    fs::write("output/clock.ppm", canvas.to_ppm()).expect("Unable to write file");
}
