use image::{ImageBuffer, Rgb, Pixel};
use num::{One, Zero, ToPrimitive};
use num::bigint::BigInt;
use num::complex::Complex;

use error::{Error, Result};


const X_OFFSET: f64 = -2.0;
const Y_OFFSET: f64 = -2.0;


#[derive(Debug)]
pub struct Tile {
    min: Complex<f64>,
    max: Complex<f64>
}


impl Tile {
    /// Returns a tile into the complex plane
    ///
    /// # Arguments:
    ///
    /// * `min` - A complex float that represents the lower left corner of the tile
    /// * `max` - a complex float that represent the upper right corner of the tile
    pub fn new(min: Complex<f64>, max: Complex<f64>) -> Tile {
        Tile { min: min, max: max }
    }

    /// Returns a tile for x, y coordinate bounds
    ///
    /// # Arguments:
    ///
    /// * `x_min` - float representing the left boundary of the tile
    /// * `y_min` - float representing the bottom boundary of the tile
    /// * `x_max` - float representing the right boundary of the tile
    /// * `y_max` - float representing the top boundary of the tile
    pub fn from_bounds(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Tile {
        let min = Complex::new(x_min, y_min);
        let max = Complex::new(x_max, y_max);
        Tile::new(min, max)
    }

    /// Returns a tile from tms coordinates.  Global tms coordinates 0/0/0 corresponds to (-2-2i,
    /// 2+2i).
    ///
    /// # Arguments:
    ///
    /// * `x` - x coordinate of tile at given zoom level
    /// * `y` - y coordiate of tile at given zoom level.  Note: by universal convention this is
    /// taken to be the negative of the y value in the tms spec.
    /// * `z` - the zoom level of the tile
    pub fn from_tms(x: u64, y: u64, z: u64) -> Tile {
        // tile size is 4.0 * 2 ** -z == 0.5 ** (z - 2)
        let tile_size: f64 = if (z >= 2) { 0.5.powi(z - 2) } else { 2.0.powi(2 - z) }

        // x_min = X_OFFSET + tile_size * x
        // y_min = Y_OFFSET + tile_size * y
        let x_min = X_OFFSET + tile_size * (x as f64);
        let y_min = Y_OFFSET + tile_size * (y as f64);
        Tile::from_bounds(x_min, y_min, x_min + tile_size, y_min + tile_size)
    }

    /// Returns (x_min, y_min, x_max, y_max) bounds of a tile
    ///
    /// # Arguments:
    /// * `&self` - tile whose bounds are returned
    pub fn bounds(&self) -> (f64, f64, f64, f64) { (self.min.re, self.min.im, self.max.re, self.max.im) }

    /// Returns the children of a tile at the next zoom level
    ///
    /// # Arguments:
    /// * `&self` - tile whose children will be returned
    pub fn children(&self, zoom: ) -> (Tile, Tile, Tile, Tile) {
    }

    /// Returns the parent of this tile at the previous zoom level
    ///
    /// # Arguments:
    /// * `&self` - tile whose parent will be returned
    pub fn parent(&self, zoom: ) -> Tile {
    }

    pub fn to_array(&self, resolution: usize, buffer: mut& [f32]) -> {
    }

    pub fn to_ndarray(&self, resolution: usize, buffer: mut& [f32]) -> {
    }

    pub fn to_image_buffer(&self, resolution: usize, buffer: mut& ImageBuffer) -> {
    }
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
