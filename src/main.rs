extern crate docopt;
extern crate image;
extern crate num;

mod mandelbrot;

use docopt::Docopt;
use image::{ImageLuma8, PNG};
use num::Complex;

use std::fs::File;
use std::path::Path;

use mandelbrot::Bounds;

const USAGE: &'static str = "
mandelbrot: fun with fractals

Usage:
  mandelbrot render <file>
  mandelbrot help

Arguments:
  -h, --help  Print this usage
";


fn main() {
    let docopt = Docopt::new(USAGE).unwrap_or_else(|e| e.exit());
    let argv_map = docopt.parse().unwrap_or_else(|e| e.exit());

    if argv_map.get_bool("render") {
        let fname = argv_map.get_str("<file>");
        render(fname)
    }
}

fn render(fname: &str) {
    let bounds = Bounds::new(Complex::new(-2.0, -2.0), Complex::new(2.0, 2.0));
    let img = mandelbrot::render(bounds, 128);
    let ref mut fout = File::create(&Path::new(fname)).unwrap();
    let _ = ImageLuma8(img).save(fout, PNG);
}
