use crate::tuple::Color;
use num_traits::cast;
use num_traits::Float;
use std::fmt::Display;
use std::vec::Vec;

struct Canvas<T> {
    width: usize,
    height: usize,
    pixels: Vec<Vec<T>>,
}

impl<T: Float + Display> Canvas<Color<T>> {
    fn new(width: usize, height: usize) -> Canvas<Color<T>> {
        Canvas::<Color<T>> {
            width,
            height,
            pixels: vec![vec!(Color::<T>::default(); width); height],
        }
    }

    fn write_pixel(&mut self, y: usize, x: usize, c: Color<T>) {
        self.pixels[x][y] = c
    }

    fn to_ppm(&self) -> String {
        let mut ppm: String = format!("P3\n{} {}\n{}\n", self.width, self.height, u8::MAX);
        const LINE_LEN_LIMIT: usize = 70;
        for r in &self.pixels {
            let mut line_len = 0;
            for c in r {
                let max: T = cast(u8::MAX).unwrap();
                for cc in [c.red, c.green, c.blue] {
                    let str = (cc * max)
                        .round()
                        //.clamp(T::zero(), max) is not available
                        .max(T::zero())
                        .min(max)
                        .to_string();
                    if line_len + str.len() > LINE_LEN_LIMIT {
                        ppm.pop();
                        ppm.push('\n');
                        line_len = 0;
                    }
                    ppm += str.as_str();
                    ppm.push(' ');
                    line_len += str.len() + 1;
                }
            }
            ppm.pop();
            ppm.push('\n');
        }
        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut canvas = Canvas::<Color<f32>>::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels[0][0], Color::<f32>::default());
        let red = Color::<f32>::new(1., 0., 0.);
        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.pixels[3][2], red);
    }

    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::<Color<f32>>::new(5, 3);
        canvas.write_pixel(0, 0, Color::<f32>::new(1.5, 0., 0.));
        canvas.write_pixel(2, 1, Color::<f32>::new(0., 0.5, 0.));
        canvas.write_pixel(4, 2, Color::<f32>::new(-0.5, 0., 1.));
        assert_eq!(
            canvas.to_ppm(),
            r"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"
        );
    }

    #[test]
    fn to_ppm_long_lines() {
        let mut canvas = Canvas::<Color<f32>>::new(10, 2);
        canvas
            .pixels
            .fill(vec![Color::<f32>::new(1., 0.8, 0.6); 10]);
        assert_eq!(
            canvas.to_ppm(),
            r"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
"
        );
    }
}
