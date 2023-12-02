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

    let answer_to_part_one = part_one
        .solve(&input)
        .context("encountered an error while solving part one")?;

    println!("Part One: {}", answer_to_part_one);

    let answer_to_part_two = part_two
        .solve(&input)
        .context("encountered an error while solving part two")?;

    println!("Part Two: {}", answer_to_part_two);

    Ok(())
}
