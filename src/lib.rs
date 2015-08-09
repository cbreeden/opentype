//! A parser for OpenType fonts.

use std::{io, result};

/// An error.
pub type Error = io::Error;

/// A result.
pub type Result<T> = result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (
        return Err(::Error::new(::std::io::ErrorKind::Other, $message));
    );
);

#[macro_use]
mod compound;

mod band;
mod font;
mod utils;

pub mod consts;
pub mod structs;
pub mod types;

pub use font::Font;

#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::path::PathBuf;

    pub fn open(name: &str) -> File {
        let path = PathBuf::from("tests/fixtures").join(name);
        assert!(fs::metadata(&path).is_ok());
        File::open(&path).unwrap()
    }
}
