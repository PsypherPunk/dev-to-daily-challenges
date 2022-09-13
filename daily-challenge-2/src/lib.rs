pub fn diamond(middle: isize) -> Option<String> {
    if middle < 0 || middle % 2 == 0 {
        None
    } else {
        let middle = middle as usize;
        let middle_index = (middle - 1) / 2;
        let middle_line = "*".repeat(middle);

        let head = (0..middle_index)
            .map(|index| {
                let asterisks = "*".repeat((2 * index) + 1);

                format!("{: ^1$}", asterisks, &middle)
            })
            .collect::<Vec<_>>();
        let tail = head.clone().into_iter().rev().collect::<Vec<_>>();

        let output = [head, vec![middle_line], tail].concat().join("\n");

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative() {
        assert_eq!(None, diamond(-1));
    }

    #[test]
    fn even() {
        assert_eq!(None, diamond(2));
    }

    #[test]
    fn valid() {
        assert_eq!(Some("*".to_owned()), diamond(1));
        assert_eq!(Some(" * \n***\n * ".to_owned()), diamond(3));
        assert_eq!(
            Some("  *  \n *** \n*****\n *** \n  *  ".to_owned()),
            diamond(5)
        );
    }
}
