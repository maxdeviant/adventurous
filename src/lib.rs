//! Adventurous provides utilities to assist you in solving
//! [Advent of Code](https://adventofcode.com) puzzles.
//!
//! [![Crates.io](https://img.shields.io/crates/v/adventurous.svg)](https://crates.io/crates/adventurous)
//! [![Docs.rs](https://docs.rs/adventurous/badge.svg)](https://docs.rs/adventurous)
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
//! use adventurous::Input;
//! use anyhow::Result;
//!
//! fn part_one(input: &Input) -> Result<usize> {
//!     Ok(input
//!         .lines()
//!         .map(|line| {
//!             // Do something with the line...
//!             line.parse::<usize>()
//!         })
//!         .collect::<Result<Vec<_>, _>>()?
//!         .into_iter()
//!         .sum())
//! }
//!
//! fn part_two(_input: &Input) -> Result<usize> {
//!     todo!()
//! }
//!
//! fn main() -> Result<()> {
//!     adventurous::run("input.txt", part_one, part_two)
//! }
//! ```

mod geometry;
mod input;
mod runner;
mod solve;
mod strings;

pub use geometry::*;
pub use input::*;
pub use runner::*;
pub use solve::*;
pub use strings::*;
