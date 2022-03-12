use crate::light::Light;
use crate::sphere::reflect;
use cgmath::{BaseFloat, InnerSpace, Point3, Vector3};
use rgb::RGB;

#[derive(Debug, PartialEq)]
pub(crate) struct Material<T> {
    color: RGB<T>,
    ambient: T,
    diffuse: T,
    specular: T,
    shininess: T,
}

impl<T: BaseFloat + Default> Material<T> {
    pub fn defaults() -> Material<T> {
        Material::<T> {
            color: RGB::new(T::one(), T::one(), T::one()),
            ambient: T::from(0.1).unwrap(),
            diffuse: T::from(0.9).unwrap(),
            specular: T::from(0.9).unwrap(),
            shininess: T::from(200.).unwrap(),
        }
    }

    pub fn lighting(
        &self,
        light: Light<T>,
        point: Point3<T>,
        eyev: Vector3<T>,
        normalv: Vector3<T>,
    ) -> RGB<T> {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let mut diffuse = RGB::default();
        let mut specular = RGB::default();
        let light_dot_normal = lightv.dot(normalv);
        if light_dot_normal >= T::zero() {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = reflect(-lightv, normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye > T::zero() {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

mod tests {
    use super::*;
    use cgmath::{assert_relative_eq, EuclideanSpace};

    #[test]
    fn lighting() {
        assert_eq!(
            Material::defaults().lighting(
                Light::new(Point3::new(0., 0., -10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                -Vector3::unit_z(),
                -Vector3::unit_z()
            ),
            RGB::new(1.9, 1.9, 1.9)
        );
    }
}
