use crate::light::Light;
use crate::sphere::reflect;
use cgmath::{BaseFloat, InnerSpace, Point3, Vector3};
use derive_more::Constructor;
use rgb::RGB;

#[derive(Constructor, Clone, Debug, PartialEq)]
pub struct Material<T> {
    pub color: RGB<T>,
    pub ambient: T,
    pub diffuse: T,
    pub specular: T,
    pub shininess: T,
}

impl<T: BaseFloat + Default> Default for Material<T> {
    fn default() -> Material<T> {
        Material::<T> {
            color: RGB::new(T::one(), T::one(), T::one()),
            ambient: T::from(0.1).unwrap(),
            diffuse: T::from(0.9).unwrap(),
            specular: T::from(0.9).unwrap(),
            shininess: T::from(200.).unwrap(),
        }
    }
}

impl<T: BaseFloat + Default> Material<T> {
    pub fn lighting(
        &self,
        light: Light<T>,
        point: Point3<T>,
        eyev: Vector3<T>,
        normalv: Vector3<T>,
        in_shadow: bool
    ) -> RGB<T> {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let mut diffuse = RGB::default();
        let mut specular = RGB::default();
        let light_dot_normal = lightv.dot(normalv);
        if !in_shadow && light_dot_normal >= T::zero() {
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
    use std::f32::consts::FRAC_1_SQRT_2;

    use super::*;
    use approx::assert_relative_eq;
    use cgmath::EuclideanSpace;

    #[test]
    fn lighting() {
        // Lighting with eye between light and surface
        assert_eq!(
            Material::default().lighting(
                Light::new(Point3::new(0., 0., -10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                -Vector3::unit_z(),
                -Vector3::unit_z(),
                false
            ),
            RGB::new(1.9, 1.9, 1.9)
        );
        // Lighting with eye between light and surface, eye offset 45 degree
        assert_eq!(
            Material::default().lighting(
                Light::new(Point3::new(0., 0., -10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                Vector3::new(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                -Vector3::unit_z(),
                false
            ),
            RGB::new(1., 1., 1.)
        );
        // Lighting with eye opposite surface, light offset 45 degree
        assert_relative_eq!(
            Material::default().lighting(
                Light::new(Point3::new(0., 10., -10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                -Vector3::unit_z(),
                -Vector3::unit_z(),
                false
            ),
            RGB::new(0.7364, 0.7364, 0.7364),
            max_relative = 0.00001
        );

        // Lighting with eye in the path of the reflection vector
        assert_relative_eq!(
            Material::default().lighting(
                Light::new(Point3::new(0., 10., -10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                Vector3::new(0., -FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
                -Vector3::unit_z()
                    ,false
            ),
            RGB::new(1.6364, 1.6364, 1.6364),
            max_relative = 0.00001
        );
        // Lighting with the light behind the surface
        assert_eq!(
            Material::default().lighting(
                Light::new(Point3::new(0., 0., 10.), RGB::new(1., 1., 1.)),
                Point3::origin(),
                -Vector3::unit_z(),
                -Vector3::unit_z()
                    ,false
            ),
            RGB::new(0.1, 0.1, 0.1)
        );
    }
}
