use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The input for an Advent of Code puzzle.
#[derive(Debug)]
pub struct Input(String);

impl Input {
    /// Returns a new [`Input`] from the given [`String`].
    pub fn new(input: String) -> Self {
        Self(input)
    }

    /// Returns a new [`Input`] containing the contents of the file at the
    /// given path.
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self(contents))
    }

    /// Returns the raw input.
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Returns an iterator over the lines in the [`Input`].
    pub fn lines(&self) -> std::str::Lines<'_> {
        self.0.lines()
    }
}
