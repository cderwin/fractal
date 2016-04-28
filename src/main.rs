extern crate argparse;
extern crate image;
extern crate num;

use argparse::{ArgumentParser, Store};
use image::{ImageLuma8, PNG};
use std::fs::File;
use std::path::Path;

mod mandelbrot;

fn main() {
    let fname = parse_args();
    let img = mandelbrot::render();
    let ref mut fout = File::create(&Path::new(&fname)).unwrap();
    let _ = ImageLuma8(img).save(fout, PNG);
}

fn parse_args() -> String {
    let mut file = "file".to_string();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Render the mandelbrot set");
        parser.refer(&mut file)
            .add_argument("filename", Store, "File to write the image to")
            .required();
        parser.parse_args_or_exit();
    }
    file
}
