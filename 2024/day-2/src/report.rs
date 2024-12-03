pub fn is_safe(report: Vec<i32>) -> bool {
    report.windows(2).all(|w| {
        let diff = if is_increasing(&report) {
            w[1] - w[0]
        } else {
            w[0] - w[1]
        };
        (1..=3).contains(&diff)
    })
}

fn is_increasing(report: &[i32]) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_increasing() {
        assert_eq!(is_safe(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn test_is_safe_increasing_by_two() {
        assert_eq!(is_safe(vec![1, 3, 5, 7, 9]), true);
    }

    #[test]
    fn test_is_safe_decreasing_by_two() {
        assert_eq!(is_safe(vec![9, 7, 5, 3, 1]), true);
    }

    #[test]
    fn test_is_safe_increasing_by_three() {
        assert_eq!(is_safe(vec![1, 4, 7, 10, 13]), true);
    }

    #[test]
    fn test_is_safe_decreasing_by_three() {
        assert_eq!(is_safe(vec![13, 10, 7, 4, 1]), true);
    }

    #[test]
    fn test_is_safe_decreasing() {
        assert_eq!(is_safe(vec![5, 4, 3, 2, 1]), true);
    }

    #[test]
    fn test_is_unsafe_big_increase() {
        assert_eq!(is_safe(vec![1, 2, 7, 8, 9]), false);
    }

    #[test]
    fn test_is_unsafe_big_decrease() {
        assert_eq!(is_safe(vec![9, 7, 6, 2, 1]), false);
    }

    #[test]
    fn test_is_unsafe_increasing_decreasing() {
        assert_eq!(is_safe(vec![1, 3, 2, 4, 5]), false);
    }

    #[test]
    fn test_is_unsafe_not_increasing_or_decreasing() {
        assert_eq!(is_safe(vec![8, 6, 4, 4, 1]), false);
    }
}
