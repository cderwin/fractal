#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate docopt;
extern crate image;
extern crate num;
extern crate rocket;

mod error;
mod mandelbrot;

use docopt::Docopt;
use image::ImageLuma8;
use num::Complex;
use rocket::response::{Response, NamedFile};
use rocket::http::ContentType;

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use error::Result;
use mandelbrot::Bounds;

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
        render(fname).unwrap();
    }

    if argv_map.get_bool("serve") {
        serve();
    }
}

#[get("/render/<z>/<y>/<x>")]
fn render_route<'a>(x: i32, y: i32, z: i32) -> Result<Response<'a>> {
    let bounds = Bounds::from_crs(x, y, z);
    let img = mandelbrot::render(bounds, 128);
    let mut buffer = io::Cursor::new(Vec::new());
    let image = ImageLuma8(img);
    image.save(&mut buffer, image::PNG)?;

    Ok(Response::build()
        .header(ContentType::PNG)
        .sized_body(buffer)
        .finalize())
}


fn render(fname: &str) -> Result<()> {
    let bounds = Bounds::new(Complex::new(-2.0, -2.0), Complex::new(2.0, 2.0));
    let img = mandelbrot::render(bounds, 128);
    let ref mut fout = File::create(&Path::new(fname))?;
    ImageLuma8(img).save(fout, image::PNG)?;
    Ok(())
}

#[get("/static/<path..>", rank=5)]
fn static_files(path: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(path))
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

fn serve() {
    rocket::ignite().mount("/", routes![index, static_files, render_route]).launch();
}
