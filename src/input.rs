use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The input for an Advent of Code puzzle.
#[derive(Debug)]
pub struct Input(String);

impl Input {
    /// Returns a new [`Input`] from the given [`String`].
    pub fn new(input: String) -> Self {
        Self(input)
    }

    /// Returns a new [`Input`] containing the contents of the file at the
    /// given path.
    pub fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self(contents))
    }

    /// Returns the raw input.
    pub fn raw(&self) -> &str {
        &self.0
    }

    /// Returns an iterator over the lines in the [`Input`].
    pub fn lines(&self) -> std::str::Lines<'_> {
        self.0.lines()
    }

    /// Traverses each line in the [`Input`] with the given function.
    pub fn traverse<T, E>(
        &self,
        f: impl FnMut(&str) -> Result<T, E>,
    ) -> Result<impl Iterator<Item = T>, E> {
        Ok(self
            .lines()
            .map(f)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter())
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl From<String> for Input {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    pub fn test_traverse_with_oks() {
        let input = Input::from(indoc! {"
            1
            2
            3
            4
            5
        "});

        assert_eq!(
            input
                .traverse(|line| line.parse::<usize>())
                .map(|iter| iter.sum::<usize>()),
            Ok(1 + 2 + 3 + 4 + 5)
        );
    }

    #[test]
    pub fn test_traverse_with_errs() {
        let input = Input::from(indoc! {"
            1
            2
            oops
            4
            5
        "});

        assert_eq!(
            input
                .traverse(|line| line.parse::<usize>())
                .map(|iter| iter.sum::<usize>())
                .is_err(),
            true
        );
    }
}
