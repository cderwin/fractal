use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::result;

use image;
use num::bigint::ParseBigIntError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Image(image::ImageError),
    ParseError(ParseBigIntError),
    NegativeZoom,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref error) => write!(f, "IO Error: {}", error),
            Error::Image(ref error) => write!(f, "Image Error: {}", error),
            Error::ParseError(ref error) => write!(f, "Parse error: {}", error),
            Error::NegativeZoom => write!(f, "Zoom level must be non-negative"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref err) => err.description(),
            Error::Image(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
            Error::NegativeZoom => "Negative zoom",

        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IO(ref err) => Some(err),
            Error::Image(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
            Error::NegativeZoom => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Error {
        Error::Image(err)
    }
}

impl From<ParseBigIntError> for Error {
    fn from(err: ParseBigIntError) -> Error {
        Error::ParseError(err)
    }
}
