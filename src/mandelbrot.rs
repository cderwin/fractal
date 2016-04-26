use image::{ImageBuffer, Luma};
use num::complex::Complex;
use rustc_serialize::Json;
use std::cmp::min;
use std::collections::HashMap;


pub struct Bounds {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32
}

impl Bounds {
    fn min(&self) -> Complex<f32> {
        Complex::new(self.x_min, self.y_min)
    }

    fn max(&self) -> Complex<f32> {
        Complex::new(self.x_max, self.y_max)
    }
}

pub struct Config {
    pub max_iterations: u16,
    pub x_pixels: u32,
    pub y_pixels: u32,
    pub bounds: Bounds
}

pub impl Config {
    fn from_json(&object: Json) -> Config {
        let bounds = Bounds {
            x_min: object.find("x_min"),
            x_max: object.find("x_max"),
            y_min: object.find("y_min"),
            y_max: object.find("y_max")
        };

        Config {
            max_iterations: min(object.find("max_iterations"), 255),
            x_pixels: object.find("width"),
            y_pixels: object.find("height"),
            bounds: bounds
        }
    }
}


pub fn render(config: &Config) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut histogram = vec![0; config.max_iterations];
    let mut pixel_map = vec![vec![0u16; y_pixels]; x_pixels];
    for raw_x in range(x_pixels) {
        for raw_y in range(y_pixels) {
            let (x, y) = scale(raw_x, raw_y);
            let iters = get_iters(x, y, config.max_iterations);
            pixel_map[x][y] = iters;
            histogram[iters] += 1
        }
    }

    let total = x_pixels * y_pixels;
    let colors = histogram.map(|c| c as f32 / total as f32);
    color_image(&pixel_map, &colors)
}


fn get_iters(x: f32, y: f32, max_iters: u16) -> u8 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let mut i = 0;
    for _ in 0..max_ites {
        if z.norm() > 2 { break; }
        z = z * z + c;
        i += 1;
    }
    i
}


fn scale(x_raw: f32, y_raw: f32, cfg: &Config) -> (f32, f32) {
    let bounds = Config.bounds;
    let x_range = (bounds.x_max - bounds.x_min) / config.x_pixels;
    let y_range = (bounds.y_max - bounds.y_min) / config.y_pixels;
    let x = x_raw as f32 * x_range + bounds.x_min;
    let y = y_raw as f32 * y_range + bounds.y_min;
    (x, y)
}


fn color_image(pixel_map: &Vec<Vec<u8>>, colors: &Vec<f32>) -> ImageBiffer<Rgba<u8>, Vec<u8>> {
    let (x_range, y_range) = (pixel_map.len(), pixel_map[0].len());
    let mut img = ImageBuffer::new(x_range, y_range);
    for x in range(x_range) {
        for y in range(y_range) {
            let pixel = get_pixel(pixel_map[x][y]);
            img.set_pixel(x, y, &pixel);
        }
    }
    img


fn get_pixel(index: &u8) -> image::Pixel {}
