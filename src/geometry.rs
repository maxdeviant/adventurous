/// Returns the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
/// between two points.
pub fn manhattan_distance((a, b): (i32, i32), (c, d): (i32, i32)) -> i32 {
    (a - c).abs() + (b - d).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (2, 3)), 5)
    }
}
