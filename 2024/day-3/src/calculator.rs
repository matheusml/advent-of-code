pub fn calculate(pairs: Vec<(u32, u32)>) -> u32 {
    pairs.iter().map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(vec![(1, 2)]), 2);
        assert_eq!(calculate(vec![(1, 2), (3, 4)]), 14);
        assert_eq!(calculate(vec![(1, 2), (3, 4), (5, 6)]), 44);
    }
}
