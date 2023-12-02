use std::fmt::Display;
use std::path::Path;

use anyhow::{Context, Result};

use crate::{Input, Solve};

pub fn run(
    input_path: impl AsRef<Path> + Display,
    part_one: impl Solve,
    part_two: impl Solve,
) -> Result<()> {
    let input = Input::from_file(&input_path)
        .with_context(|| format!("failed to read puzzle input from '{}'", input_path))?;

    println!("Part One: {}", part_one.solve(&input)?);
    println!("Part Two: {}", part_two.solve(&input)?);

    Ok(())
}
