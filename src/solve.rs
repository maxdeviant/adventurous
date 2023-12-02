use std::fmt::Display;

use anyhow::Result;

use crate::Input;

/// A trait for solving an Advent of Code puzzle.
///
/// This trait is automatically implemented for functions that take a reference to
/// an [`Input`] and return a [`Result`] containing a type that implements [`Display`]:
///
/// ```
/// use adventurous::Input;
/// use anyhow::Result;
///
/// fn part_one(input: &Input) -> Result<usize> {
///     Ok(42)
/// }
/// ```
pub trait Solve {
    /// The answer to the puzzle.
    type Answer: Display;

    /// Produces an answer from the provided [`Input`].
    fn solve(&self, input: &Input) -> Result<Self::Answer>;
}

impl<A, F> Solve for F
where
    A: Display,
    F: Fn(&Input) -> Result<A>,
{
    type Answer = A;

    fn solve(&self, input: &Input) -> Result<Self::Answer> {
        self(input)
    }
}
