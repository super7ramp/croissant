/// Hardcoded Latin Script
const LETTERS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Returns the letter index in the alphabet for the given letter.
pub fn letter_at(index: usize) -> char {
    LETTERS[index]
}

/// Returns the index in the alphabet for the given letter in a [Some], or [None] if given letter
/// does not belong to the alphabet.
pub fn index_of(letter: char) -> Option<usize> {
    LETTERS.binary_search(&letter).map(Some).unwrap_or_default()
}

/// Returns `true` iff the given letter is part of the alphabet.
pub fn contains(value: char) -> bool {
    LETTERS.contains(&value)
}

/// Returns the size of the alphabet.
pub const fn letter_count() -> usize {
    LETTERS.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_contains() {
        assert!(contains('A'));
        assert!(contains('E'));
        assert!(contains('Z'));
        assert!(!contains('@'));
        assert!(!contains('&'));
        assert!(!contains('À'));
    }

    #[test]
    fn alphabet_letter_at() {
        assert_eq!('A', letter_at(0));
        assert_eq!('E', letter_at(4));
        assert_eq!('Z', letter_at(25));
    }

    #[test]
    #[should_panic]
    fn alphabet_letter_at_oob() {
        letter_at(26);
    }

    #[test]
    fn alphabet_index_of() {
        assert_eq!(Some(0), index_of('A'));
        assert_eq!(Some(4), index_of('E'));
        assert_eq!(Some(25), index_of('Z'));
        assert_eq!(None, index_of('@'));
        assert_eq!(None, index_of('&'));
        assert_eq!(None, index_of('À'));
    }

    #[test]
    fn alphabet_number_of_letters() {
        assert_eq!(26, letter_count())
    }
}
