use cgmath::Point3;
use rgb::RGB;

pub struct Light<T> {
    pub position: Point3<T>,
    pub intensity: RGB<T>,
}

impl<T> Light<T> {
    pub fn new(position: Point3<T>, intensity: RGB<T>) -> Light<T> {
        Light::<T> {
            position,
            intensity,
        }
    }
}
