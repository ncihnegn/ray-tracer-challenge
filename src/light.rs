#[derive(Clone, derive_more::Constructor, Copy, Debug)]
pub struct Light<T> {
    pub position: cgmath::Point3<T>,
    pub intensity: rgb::RGB<T>,
}
