use image::{ImageBuffer, Luma};
use num::complex::Complex;

#[derive(Debug)]
pub struct Bounds {
    min: Complex<f64>,
    max: Complex<f64>
}

impl Bounds {
    pub fn new(min: Complex<f64>, max: Complex<f64>) -> Bounds {
        Bounds { min: min, max: max }
    }

    pub fn from_crs(x: i32, y: i32, z: i32) -> Bounds {
        let increment = 4.0_f64 * 2.0_f64.powi(-z);
        let min = Complex::new(-2.0 + (x as f64) * increment, -2.0 + (y as f64) * increment);
        let max = Complex::new(-2.0 + ((x + 1) as f64) * increment, -2.0 + ((y + 1) as f64) * increment);
        Bounds::new(min, max)
    }

    pub fn re_min(&self) -> f64 { self.min.re }
    pub fn re_max(&self) -> f64 { self.max.re }
    pub fn im_min(&self) -> f64 { self.min.im }
    pub fn im_max(&self) -> f64 { self.max.im }
}

pub fn render(bounds: Bounds, max_iter: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let x_pixels = 256;
    let y_pixels = 256;

    let mut img = ImageBuffer::new(x_pixels, y_pixels);

    let x_scale = (bounds.re_max() - bounds.re_min()) / x_pixels as f64;
    let y_scale = (bounds.im_max() - bounds.im_min()) / y_pixels as f64;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x_coord = bounds.re_min() + (x as f64) * x_scale;
        let y_coord = bounds.im_min() + (y as f64) * y_scale;

        let c = Complex::new(x_coord, y_coord);
        let mut z = Complex::new(0.0, 0.0);
        let mut i = 0;

        for _ in 0..max_iter {
            if z.norm() > 2.0 { break; }
            z = z * z + c;
            i += 1;
        }

        *pixel = Luma([i as u8]);
    }

    img
}
