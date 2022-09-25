pub fn vowel_counter(input: &str) -> usize {
    input
        .chars()
        .filter(|c| matches!(c, 'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(vowel_counter(""), 0);
    }

    #[test]
    fn single() {
        assert_eq!(vowel_counter("a"), 1);
        assert_eq!(vowel_counter("e"), 1);
        assert_eq!(vowel_counter("i"), 1);
        assert_eq!(vowel_counter("o"), 1);
        assert_eq!(vowel_counter("u"), 1);
        assert_eq!(vowel_counter("A"), 1);
        assert_eq!(vowel_counter("E"), 1);
        assert_eq!(vowel_counter("I"), 1);
        assert_eq!(vowel_counter("O"), 1);
        assert_eq!(vowel_counter("U"), 1);
    }

    #[test]
    fn multiple() {
        assert_eq!(vowel_counter("aa"), 2);
    }
}
