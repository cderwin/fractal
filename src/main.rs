extern crate argparse;
extern crate image;
extern crate iron;
extern crate num;

use argparse::{ArgumentParser, Store};
use image::{ImageLuma8, PNG};
use iron::prelude::*;
use iron::status;
use num::Complex;
use std::fs::File;
use std::path::Path;

mod mandelbrot;



fn main() {
    let (fname, img_cfg) = parse_args();
    let img = mandelbrot::render(&img_cfg);
    let ref mut fout = File::create(&Path::new(&fname)).unwrap();
    let _ = ImageLuma8(img).save(fout, PNG);
}

fn handler(request: &mut Request) -> IronResult<Response> {
    let params = query::parse(request.url.query);
    let params = mandelbrot::Config::from_json(&params);



fn parse_args() -> (String, mandelbrot::Config) {
    let mut file = "file".to_string();
    let mut bounds = Bounds {x_min: -2.0, x_max: 2.0, y_min: -2.0, y_max: 2.0};
    let mut cfg = mandelbrot::Config {
        max_iterations: 800u16,
        x_pixels: 800,
        y_pixels: 800,
        min: bounds.min(),
        max: bounds.max()
    };

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Render the mendelbrot set");
        parser.refer(&mut file)
            .add_argument("filename", Store, "File to write the image to")
            .required();

        parser.refer(&mut cfg.max_iterations)
            .add_option(&["--max-iterations", "--iter"], Store,
                        "Number of iterations used to determine rate of convergence");

        parser.refer(&mut cfg.x_pixels)
            .add_option(&["--x-pixels", "--xp"], Store,
                        "Number of pixels across the image (horizontal)");

        parser.refer(&mut cfg.y_pixels)
            .add_option(&["--y-pixels", "--yp"], Store,
                        "Number of pixels up the image (vertical)");

        parser.refer(&mut bounds.x_min)
            .add_option(&["--x-min", "--xmin"], Store,
                        "Minimum bound of the real axis");

        parser.refer(&mut bounds.x_max)
            .add_option(&["--x-max", "--xmax"], Store,
                        "Maximum bound of the real axis");

        parser.refer(&mut bounds.y_min)
            .add_option(&["--y-min", "--ymin"], Store,
                        "Minimum bound of the imaginary axis");

        parser.refer(&mut bounds.y_max)
            .add_option(&["--y-max", "--ymax"], Store,
                        "Maximum bound of the imaginary axis");

        parser.parse_args_or_exit();
    }

    cfg.min = bounds.min();
    cfg.max = bounds.max();
    (file, cfg)
}
