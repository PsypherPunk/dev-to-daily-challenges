pub fn peel(string: &str) -> Option<&str> {
    if string.len() <= 2 {
        None
    } else {
        Some(&string[1..(string.len() - 1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_short() {
        assert_eq!(peel(""), None);
        assert_eq!(peel("a"), None);
        assert_eq!(peel("ab"), None);
    }

    #[test]
    fn valid() {
        assert_eq!(peel("abc"), Some("b"));
        assert_eq!(peel("abcd"), Some("bc"));
    }
}
