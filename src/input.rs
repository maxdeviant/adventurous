use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The input for an Advent of Code puzzle.
#[derive(Debug)]
pub struct Input {
    pub value: String,
}

impl Input {
    /// Returns a new [`Input`] from the given [`String`].
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Returns a new [`Input`] containing the contents of the file at the
    /// given path.
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self { value: contents })
    }
}
