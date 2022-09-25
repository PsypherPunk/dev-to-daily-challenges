struct Expense {
    amount: f64,
    running_balance: f64,
}

pub fn balance_chequebook(chequebook: &str) -> Result<String, String> {
    let expenses = chequebook.lines().try_fold(Vec::new(), |mut acc, line| {
        let value = line
            .split_whitespace()
            .last()
            .ok_or_else(|| "invalid value".to_owned())?;

        let value = value.parse::<f64>().map_err(|e| e.to_string())?;

        match acc.len() {
            0 => acc.push(Expense {
                amount: 0.0,
                running_balance: value,
            }),
            _ => acc.push(Expense {
                amount: value,
                running_balance: acc.last().ok_or("empty vec")?.running_balance - value,
            }),
        }

        Ok::<Vec<Expense>, String>(acc)
    })?;

    let total_expenses = expenses.iter().map(|expense| expense.amount).sum::<f64>();

    let output = [
        chequebook
            .lines()
            .zip(&expenses)
            .enumerate()
            .map(|(i, (line, expense))| match i {
                0 => format!("Original_Balance: {}", line),
                _ => format!("{} Balance {:.2}", line, expense.running_balance),
            })
            .collect::<Vec<_>>(),
        vec![
            format!("Total expense {:.2}", total_expenses),
            format!(
                "Average expense {:.2}",
                total_expenses / (expenses.len() - 1) as f64
            ),
        ],
    ]
    .concat()
    .join("\n");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHEQUEBOOK: &str = r#"1000.00
125 Market 125.45
126 Hardware 34.95
127 Video 7.45
128 Book 14.32
129 Gasoline 16.10"#;

    #[test]
    fn test_sample() {
        let balanced = balance_chequebook(CHEQUEBOOK);

        let expected = r#"Original_Balance: 1000.00
125 Market 125.45 Balance 874.55
126 Hardware 34.95 Balance 839.60
127 Video 7.45 Balance 832.15
128 Book 14.32 Balance 817.83
129 Gasoline 16.10 Balance 801.73
Total expense 198.27
Average expense 39.65"#;

        assert_eq!(Ok(expected.to_owned()), balanced);
    }
}
