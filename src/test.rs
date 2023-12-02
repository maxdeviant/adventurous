use std::fmt::Display;
use std::path::Path;

use anyhow::{Context, Result};
use pretty_assertions::assert_eq;

use crate::{Input, Solve};

/// Tests the provided solution against the input and asserts that it has the
/// the specified answer.
pub fn test_solution<S: Solve>(
    input_path: impl AsRef<Path> + Display,
    solution: S,
    expected: S::Answer,
) -> Result<()> {
    let input = Input::from_file(&input_path)
        .with_context(|| format!("failed to read puzzle input from '{}'", input_path))?;

    let answer = solution.solve(&input)?;

    assert_eq!(answer, expected);

    Ok(())
}

/// Emits a test case to test the solution for part one of the puzzle.
///
/// Use this once you've correctly solved part one of the puzzle to ensure
/// your solution doesn't regress as you modify it.
///
/// # Examples
///
/// ```
/// use adventurous::test_part_one_solution;
///
/// test_part_one_solution!("input.txt", 42);
/// ```
#[macro_export]
macro_rules! test_part_one_solution {
    ($input_path:literal, $expected:literal) => {
        #[test]
        fn test_part_one_solution() -> anyhow::Result<()> {
            adventurous::test::test_solution($input_path, part_one, $expected)
        }
    };
}

/// Emits a test case to test the solution for part two of the puzzle.
///
/// Use this once you've correctly solved part two of the puzzle to ensure
/// your solution doesn't regress as you modify it.
///
/// # Examples
///
/// ```
/// use adventurous::test_part_two_solution;
///
/// test_part_two_solution!("input.txt", 73);
/// ```
#[macro_export]
macro_rules! test_part_two_solution {
    ($input_path:literal, $expected:literal) => {
        #[test]
        fn test_part_two_solution() -> anyhow::Result<()> {
            adventurous::test::test_solution($input_path, part_two, $expected)
        }
    };
}
