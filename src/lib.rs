mod camera;
mod canvas;
mod computation;
mod intersection;
mod light;
mod material;
mod pattern;
mod ray;
mod shape;
mod world;

#[macro_export]
macro_rules! impl_approx {
    ($ty:ident => $($type:ty)+ => $($field:tt)+) => {
        impl<T: BaseFloat> AbsDiffEq for $ty<T> {
            type Epsilon = T::Epsilon;

            #[inline]
            fn default_epsilon() -> T::Epsilon {
                T::default_epsilon()
            }

            #[inline]
            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon)
            -> bool
            {
                $(<$type>::abs_diff_eq(&self.$field, &other.$field, epsilon))&&+
            }
        }

        impl<T: BaseFloat> RelativeEq for $ty<T> {
            #[inline]
            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            #[inline]
            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $(<$type>::relative_eq(&self.$field, &other.$field, epsilon, max_relative))&&+
            }
        }

        impl<T: BaseFloat> UlpsEq for $ty<T> {
            #[inline]
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            #[inline]
            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $(<$type>::ulps_eq(&self.$field, &other.$field, epsilon, max_ulps))&&+
            }
        }
    };
}
