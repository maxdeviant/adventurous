# Adventurous

Adventurous is a companion crate to assist you in solving [Advent of Code](https://adventofcode.com) puzzles.

[![crates.io](https://img.shields.io/crates/v/adventurous.svg)](https://crates.io/crates/adventurous)
[![docs.rs](https://docs.rs/adventurous/badge.svg)](https://docs.rs/adventurous)
[![license](https://img.shields.io/crates/l/adventurous.svg)](https://github.com/maxdeviant/adventurous/blob/main/LICENSE)

## Installation

```toml
[dependencies]
adventurous = "0.2.0"
```

## Examples

### Solving a puzzle

```rust no_run
use adventurous::Input;
use anyhow::Result;

fn part_one(input: &Input) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            // Do something with the line...
            line.parse::<usize>()
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum())
}

fn part_two(_input: &Input) -> Result<usize> {
    todo!()
}

fn main() -> Result<()> {
    adventurous::run("input.txt", part_one, part_two)
}
```
