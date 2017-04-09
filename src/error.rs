use std::convert::From;
use std::error;
use std::fmt;
use std::io;
use std::result;

use image;

pub type Result<T> = result::Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    IO(io::Error),
    Image(image::ImageError),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::IO(ref error) => write!(f, "IO Error: {}", error),
            ApiError::Image(ref error) => write!(f, "Image Error: {}", error),
        }
    }
}

impl error::Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::IO(ref err) => err.description(),
            ApiError::Image(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ApiError::IO(ref err) => Some(err),
            ApiError::Image(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(err: io::Error) -> ApiError {
        ApiError::IO(err)
    }
}

impl From<image::ImageError> for ApiError {
    fn from(err: image::ImageError) -> ApiError {
        ApiError::Image(err)
    }
}
