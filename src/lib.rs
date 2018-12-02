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
//! adventurous = "0.0.2"
//! ```

mod input;

pub use self::input::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
