#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate docopt;
extern crate image;
extern crate num;
extern crate rocket;

mod error;
mod mandelbrot;
mod routes;

use docopt::Docopt;
use image::ImageRgb8;
use num::Complex;

use std::fs::File;
use std::path::Path;

use error::Result;
use mandelbrot::{Bounds, Gradient};
use routes::RenderOptions;

const USAGE: &'static str = "
mandelbrot: fun with fractals

Usage:
  mandelbrot render <file>
  mandelbrot serve
  mandelbrot help

Arguments:
  -h, --help  Print this usage
";


fn main() {
    let docopt = Docopt::new(USAGE).unwrap_or_else(|e| e.exit());
    let argv_map = docopt.parse().unwrap_or_else(|e| e.exit());

    if argv_map.get_bool("render") {
        let fname = argv_map.get_str("<file>");
        render_to_file(fname).unwrap();
    }

    if argv_map.get_bool("serve") {
        serve();
    }
}


fn render_to_file(fname: &str) -> Result<()> {
    let bounds = Bounds::new(Complex::new(-2.0, -2.0), Complex::new(2.0, 2.0));
    let gradient = Gradient::new(RenderOptions::to_color(0xf9690e), RenderOptions::to_color(0x1f3a93));
    let img = mandelbrot::render(bounds, gradient, 128);
    let ref mut fout = File::create(&Path::new(fname))?;
    ImageRgb8(img).save(fout, image::PNG)?;
    Ok(())
}


fn serve() {
    rocket::ignite().mount("/", routes![routes::index, routes::static_files, routes::render]).launch();
}
