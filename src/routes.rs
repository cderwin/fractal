use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use rocket::response::{Response, NamedFile};
use rocket::http::ContentType;
use image::{self, ImageRgb8, Rgb, Pixel};
use num::BigInt;

use error::Result;
use mandelbrot::{self, Bounds, Gradient};


#[derive(FromForm)]
pub struct RenderOptions {
    max_iter: Option<u32>,
    start_gradient: Option<u32>,
    end_gradient: Option<u32>,
    pixels: Option<u32>,
    max_radius_sq: Option<f64>,
}

impl RenderOptions {
    pub fn gradient(&self) -> Gradient {
        let start = match self.start_gradient {
            Some(color) => RenderOptions::to_color(color),
            None => RenderOptions::to_color(0xf9690e)
        };

        let end = match self.end_gradient {
            Some(color) => RenderOptions::to_color(color),
            None => RenderOptions::to_color(0x1f3a93)
        };

        Gradient::new(start, end)
    }

    pub fn to_color(color: u32) -> Rgb<u8> {
        let red = (0xff & color) as u8;
        let green = (((0xff << 8) & color) >> 8) as u8;
        let blue = (((0xff << 16) & color) >> 16) as u8;
        Rgb::from_channels(red, green, blue, 0)
    }

    pub fn max_iter(&self) -> u32 {
        match self.max_iter {
            Some(num) => num,
            None => 1024
        }
    }
}


#[get("/render/<z>/<y>/<x>?<options>")]
pub fn render<'a>(x: u64, y: u64, z: u64, options: RenderOptions) -> Result<Response<'a>> {
    let bounds = Bounds::from_crs(x, y, z)?;
    let img = mandelbrot::render(bounds, options.gradient(), options.max_iter());
    let mut buffer = io::Cursor::new(Vec::new());
    ImageRgb8(img).save(&mut buffer, image::PNG)?;

    Ok(Response::build()
        .header(ContentType::PNG)
        .sized_body(buffer)
        .finalize())
}


#[get("/static/<path..>", rank=5)]
pub fn static_files(path: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(path))
}


#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
