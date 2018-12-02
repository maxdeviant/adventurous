//! Adventurous provides utilities to assist you in solving
//! [Advent of Code](https://adventofcode.com) puzzles.
//!
//! [![Crates.io](https://img.shields.io/crates/v/adventurous.svg)](https://crates.io/crates/adventurous)
//! [![Docs.rs](https://docs.rs/adventurous/badge.svg)](https://docs.rs/ggez)
//! [![Crates.io](https://img.shields.io/crates/l/adventurous.svg)](https://github.com/maxdeviant/adventurous/blob/master/LICENSE)
//!
//! ## Installation
//! ```toml
//! [dependencies]
//! adventurous = "0.0.3"
//! ```
//!
//! ## Examples
//!
//! ### Reading Puzzle Input
//!
//! ```no_run
//! extern crate adventurous;
//!
//! use adventurous::Input;
//!
//! fn main() -> std::io::Result<()> {
//!     let input = Input::from_file("input.txt")?;
//!     for line in input.value.lines() {
//!         // Do something with the line...
//!     }
//!     Ok(())
//! }
//! ```

mod geometry;
mod input;
mod strings;

pub use self::geometry::*;
pub use self::input::*;
pub use self::strings::*;
