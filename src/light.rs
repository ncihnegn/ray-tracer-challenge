use cgmath::Point3;
use derive_more::Constructor;
use rgb::RGB;

#[derive(Constructor)]
pub struct Light<T> {
    pub position: Point3<T>,
    pub intensity: RGB<T>,
}
