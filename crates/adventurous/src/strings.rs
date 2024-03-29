use std::collections::HashMap;

/// A trait for counting the number of occurrences of each character in a string.
pub trait CharCounts {
    /// Returns a [`HashMap`] containing the number of occurrences
    /// of each character in the string.
    fn char_counts(&self) -> HashMap<char, usize>;
}

impl CharCounts for str {
    fn char_counts(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for character in self.chars() {
            *counts.entry(character).or_insert(0) += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_counts() {
        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 3);
        expected.insert('o', 2);
        expected.insert(' ', 1);
        expected.insert('w', 1);
        expected.insert('r', 1);
        expected.insert('d', 1);

        assert_eq!("hello world".char_counts(), expected);
        assert_eq!(String::from("hello world").char_counts(), expected)
    }
}
