use num_traits::cast;
use num_traits::Float;
use rgb::RGB;
use std::fmt::Display;
use std::vec::Vec;

struct Canvas<T> {
    width: usize,
    height: usize,
    pixels: Vec<Vec<T>>,
}

impl<T: Float + Default + Display> Canvas<RGB<T>> {
    fn new(width: usize, height: usize) -> Canvas<RGB<T>> {
        Canvas::<RGB<T>> {
            width,
            height,
            pixels: vec![vec!(RGB::<T>::default(); width); height],
        }
    }

    fn write_pixel(&mut self, y: usize, x: usize, c: RGB<T>) {
        self.pixels[x][y] = c
    }

    fn to_ppm(&self) -> String {
        let mut ppm: String = format!("P3\n{} {}\n{}\n", self.width, self.height, u8::MAX);
        const LINE_LEN_LIMIT: usize = 70;
        for r in &self.pixels {
            let mut line_len = 0;
            for c in r {
                let max: T = cast(u8::MAX).unwrap();
                for cc in [c.r, c.g, c.b] {
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
        let mut canvas = Canvas::<RGB<f32>>::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        assert_eq!(canvas.pixels[0][0], RGB::<f32>::default());
        let r = RGB::<f32>::new(1., 0., 0.);
        canvas.write_pixel(2, 3, r);
        assert_eq!(canvas.pixels[3][2], r);
    }

    #[test]
    fn to_ppm() {
        let mut canvas = Canvas::<RGB<f32>>::new(5, 3);
        canvas.write_pixel(0, 0, RGB::<f32>::new(1.5, 0., 0.));
        canvas.write_pixel(2, 1, RGB::<f32>::new(0., 0.5, 0.));
        canvas.write_pixel(4, 2, RGB::<f32>::new(-0.5, 0., 1.));
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
        let mut canvas = Canvas::<RGB<f32>>::new(10, 2);
        canvas.pixels.fill(vec![RGB::<f32>::new(1., 0.8, 0.6); 10]);
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
