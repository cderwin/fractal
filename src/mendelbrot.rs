use image::{ImageBuffer, Luma};
use num::complex::Complex;

pub fn render() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let max_iterations = 256u16;
    let x_pixels = 1000;
    let y_pixels = 1000;

    let mut img = ImageBuffer::new(x_pixels, y_pixels);

    let x_scale = 4.0 / x_pixels as f32;
    let y_scale = 4.0 / y_pixels as f32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x_coord = x as f32 * x_scale - 2.0;
        let y_coord = y as f32 * y_scale - 2.0;

        let c = Complex::new(x_coord, y_coord);
        let mut z = Complex::new(0.0, 0.0);
        let mut i = 0;

        for _ in 0..max_iterations {
            if z.norm() > 2.0 { break; }
            z = z * z + c;
            i += 1;
        }

        *pixel = Luma([i as u8]);
    }

    img
}
