use crate::{canvas::Canvas, ray::Ray, world::World};
use cgmath::{BaseFloat, EuclideanSpace, InnerSpace, Matrix4, Point3, SquareMatrix};
use derive_more::Constructor;
use rgb::RGB;
use std::fmt::Display;

#[derive(Constructor)]
pub struct Camera<T> {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: T,
    pub transform: Matrix4<T>,
    pub half_width: T,
    pub half_height: T,
    pub pixel_size: T,
}

impl<T: BaseFloat + Default + Display> Camera<T> {
    pub fn from(hsize: usize, vsize: usize, field_of_view: T) -> Camera<T> {
        let two = T::from(2).unwrap();
        let half_view = (field_of_view / two).tan();
        let h = T::from(hsize).unwrap();
        let aspect = h / T::from(vsize).unwrap();
        let half_width = if aspect >= T::one() {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = half_width / aspect;
        Camera::<T> {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            half_width,
            half_height,
            pixel_size: half_width * two / h,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray<T> {
        let half = T::from(0.5).unwrap();
        let xoffset = (T::from(px).unwrap() + half) * self.pixel_size;
        let yoffset = (T::from(py).unwrap() + half) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let inverse = self.transform.invert().unwrap();
        let negone = T::from(-1).unwrap();
        let pixel = inverse * Point3::new(world_x, world_y, negone).to_homogeneous();
        let origin = inverse * Point3::origin().to_homogeneous();
        Ray::new(
            Point3::origin() + origin.truncate(),
            (pixel - origin).truncate().normalize(),
        )
    }

    fn render(&self, mut w: World<T>) -> Canvas<RGB<T>> {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                image.pixels[x][y] = w.color_at(ray);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{assert_abs_diff_eq, assert_relative_eq, Quaternion, Rad, Rotation3, Vector3};
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    #[test]
    fn new() {
        assert_relative_eq!(Camera::from(200, 125, FRAC_PI_2).pixel_size, 0.01);
        assert_relative_eq!(Camera::from(125, 200, FRAC_PI_2).pixel_size, 0.01);
    }

    #[test]
    fn ray_for_pixel() {
        let mut c = Camera::from(201, 101, FRAC_PI_2);
        assert_relative_eq!(
            c.ray_for_pixel(100, 50),
            Ray::new(Point3::origin(), -Vector3::unit_z())
        );
        assert_relative_eq!(
            c.ray_for_pixel(0, 0),
            Ray::new(Point3::origin(), Vector3::new(0.66519, 0.33259, -0.66851)),
            max_relative = 0.00001
        );
        {
            let point = Point3::new(0., 2., -5.);
            c.transform = Matrix4::from(Quaternion::from_angle_y(Rad(FRAC_PI_4)))
                * Matrix4::from_translation(-point.to_vec());
            assert_abs_diff_eq!(
                c.ray_for_pixel(100, 50),
                Ray::new(point, Vector3::new(FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2)),
                epsilon = 0.000001
            );
        }
    }

    #[test]
    fn render() {
        let w = World::default();
        let mut c = Camera::from(11, 11, FRAC_PI_2);
        c.transform = Matrix4::look_at_rh(
            Point3::new(0., 0., -5.),
            Point3::origin(),
            Vector3::unit_y(),
        );
        let image = c.render(w);
        approx::assert_relative_eq!(
            image.pixels[5][5],
            RGB::new(0.38066, 0.47583, 0.2855),
            max_relative = 0.0001
        );
    }
}
