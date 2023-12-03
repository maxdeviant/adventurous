use std::fmt::Display;
use std::path::Path;

use anyhow::{Context, Result};
use pretty_assertions::assert_eq;

use crate::{Input, Solve};

/// Tests the provided solution against the input and asserts that it has the
/// the specified answer.
#[doc(hidden)]
pub fn test_solution<S: Solve>(
    input_path: impl AsRef<Path> + Display,
    solution: S,
    expected: impl std::fmt::Debug + Display + PartialEq,
) -> Result<()> {
    let input = Input::from_file(&input_path)
        .with_context(|| format!("failed to read puzzle input from '{}'", input_path))?;

    let answer = solution.solve(&input)?;

    assert_eq!(answer.to_string(), expected.to_string());

    Ok(())
}
