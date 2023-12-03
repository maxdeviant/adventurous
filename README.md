# Adventurous

Adventurous is a companion crate to assist you in solving [Advent of Code](https://adventofcode.com) puzzles.

[![crates.io](https://img.shields.io/crates/v/adventurous.svg)](https://crates.io/crates/adventurous)
[![docs.rs](https://docs.rs/adventurous/badge.svg)](https://docs.rs/adventurous)
[![license](https://img.shields.io/crates/l/adventurous.svg)](https://github.com/maxdeviant/adventurous/blob/main/LICENSE)

## Installation

```toml
[dependencies]
adventurous = "0.3.0"
```

## Examples

### Solving a puzzle

```rust no_run
use adventurous::Input;
use anyhow::Result;

#[adventurous::part_one]
fn part_one(input: &Input) -> Result<usize> {
    Ok(input
        .traverse(|line| {
            // Do something with the line...
            line.parse::<usize>()
        })?
        .sum())
}

#[adventurous::part_two]
fn part_two(_input: &Input) -> Result<usize> {
    todo!()
}

fn main() -> Result<()> {
    adventurous::run("input.txt", part_one, part_two)
}
```

### Regression testing

Once a solution has been solved, you can provide the correct answer for each part using the `#[part_one]` and `#[part_two]` attributes.

Calling `test_solutions!()` inside of your `tests` module will generate regression tests that ensure the output from your solvers matches the correct answer.

```rust
use adventurous::Input;
use anyhow::Result;

#[adventurous::part_one(answer = "73")]
fn part_one(input: &Input) -> Result<usize> {
    Ok(input
        .traverse(|line| {
            // Do something with the line...
            line.parse::<usize>()
        })?
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    adventurous::test_solutions!();
}
