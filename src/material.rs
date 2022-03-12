use cgmath::BaseFloat;
use rgb::RGB;

#[derive(Debug, PartialEq)]
pub(crate) struct Material<T> {
    color: RGB<T>,
    ambient: T,
    diffuse: T,
    specular: T,
    shininess: T,
}

impl<T: BaseFloat> Material<T> {
    pub fn defaults() -> Material<T> {
        Material::<T> {
            color: RGB::new(T::one(), T::one(), T::one()),
            ambient: T::from(0.1).unwrap(),
            diffuse: T::from(0.9).unwrap(),
            specular: T::from(0.9).unwrap(),
            shininess: T::from(200.).unwrap(),
        }
    }
}
