//! Adventurous provides utilties to assist you in solving
//! [Advent of Code](https://adventofcode.com) puzzles.
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
