pub fn get_matrix() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_matrix() {
        let input = get_matrix();
        assert!(!input.is_empty());
        assert!(!input[0].is_empty()); // Check that we have at least one row with content

        // Check if we can find 'X', 'M', 'A', 'S' characters in the input
        let has_all_chars = input.iter().any(|row| {
            row.windows(4).any(|window| {
                window == &['X', 'M', 'A', 'S']
                    || window == &['M', 'A', 'S', 'X']
                    || window == &['A', 'S', 'X', 'M']
                    || window == &['S', 'X', 'M', 'A']
            })
        });
        assert!(has_all_chars, "Should contain XMAS pattern");
    }
}
