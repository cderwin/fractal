use image::{ImageBuffer, Rgb, Pixel};
use num::{One, Zero, ToPrimitive};
use num::bigint::BigInt;
use num::complex::Complex;

use error::{Error, Result};


const X_OFFSET: f64 = -2.0;
const Y_OFFSET: f64 = -2.0;

#[derive(Debug)]
pub struct Bounds {
    min: Complex<f64>,
    max: Complex<f64>
}


impl Bounds {
    pub fn new(min: Complex<f64>, max: Complex<f64>) -> Bounds {
        Bounds { min: min, max: max }
    }

    pub fn from_floats(re_min: f64, im_min: f64, re_max: f64, im_max: f64) -> Bounds {
        let min = Complex::new(re_min, im_min);
        let max = Complex::new(re_max, im_max);
        Bounds::new(min, max)
    }

    pub fn from_crs(x: u64, y: u64, z: u64) -> Result<Bounds> {
        if z < 0 {
            return Err(Error::NegativeZoom)
        }

        let mut ctr = z;
        let scaling_factor = 0.5 as f64;
        let mut increment = 4.0 as f64;
        while ctr > 0 {
            increment = increment * scaling_factor;
            ctr = ctr - 1;
        }

        // x * incr, y * incr
        // (x + 1) * incr, (y + 1) * incr
        let min = Complex::new((x as f64) * increment + X_OFFSET, (y as f64) * increment + Y_OFFSET);
        let max = Complex::new(((x + 1) as f64) * increment + X_OFFSET, ((y + 1) as f64) * increment + Y_OFFSET);
        Ok(Bounds::new(min, max))
    }

    pub fn re_min(&self) -> f64 { self.min.re }
    pub fn re_max(&self) -> f64 { self.max.re }
    pub fn im_min(&self) -> f64 { self.min.im }
    pub fn im_max(&self) -> f64 { self.max.im }
}


pub fn render(bounds: Bounds, gradient: Gradient, max_iter: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (x_pixels, y_pixels) = (256, 256);
    let mut img = ImageBuffer::new(x_pixels, y_pixels);

    let x_scale = (bounds.re_max() - bounds.re_min()) / (x_pixels as f64);
    let y_scale = (bounds.im_max() - bounds.im_min()) / (y_pixels as f64);

    let two = 2.0 as f64;
    let four = 4.0 as f64;

    for (x_coord, y_coord, pixel) in img.enumerate_pixels_mut() {
        let re_start = bounds.re_min() + (x_coord as f64) * x_scale;
        let im_start = bounds.im_min() + (y_coord as f64) * y_scale;

        let (mut re, mut im) = (re_start, im_start);

        let mut i = 1;
        while i < max_iter {
            let re_sq = re * re;
            let im_sq = im * im;

            if re_sq + im_sq > four { break; }

            im = im * re * two + im_start;
            re = re_sq - im_sq + re_start;
            i += 1;
        }

        *pixel = gradient.value((i as f64)/(max_iter as f64));
    }

    img
}


#[derive(Debug)]
pub struct Gradient {
    min: Rgb<u8>,
    max: Rgb<u8>,
}


impl Gradient {
    pub fn new(min: Rgb<u8>, max: Rgb<u8>) -> Gradient {
        Gradient {
            min: min,
            max: max,
        }
    }

    fn value(&self, value: f64) -> Rgb<u8> {
        let min_channels = self.min.channels();
        let max_channels = self.max.channels();

        let red   = ((1.0 - value) * (min_channels[0] as f64) + value * (max_channels[0] as f64)) as u8;
        let green = ((1.0 - value) * (min_channels[1] as f64) + value * (max_channels[1] as f64)) as u8;
        let blue  = ((1.0 - value) * (min_channels[2] as f64) + value * (max_channels[2] as f64)) as u8;

        Rgb([red, green, blue])
    }
}
