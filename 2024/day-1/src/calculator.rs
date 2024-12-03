pub fn calculate_difference(array_1: &[i32], array_2: &[i32]) -> i32 {
    let mut result = 0;

    for i in 0..array_1.len() {
        result += (array_2[i] - array_1[i]).abs();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_difference() {
        let array_1 = vec![1, 2, 3];
        let array_2 = vec![4, 5, 6];
        assert_eq!(calculate_difference(&array_1, &array_2), 9);
    }

    #[test]
    fn test_calculate_difference_with_negatives() {
        let array_1 = vec![-1, -2, -3];
        let array_2 = vec![1, 2, 3];
        assert_eq!(calculate_difference(&array_1, &array_2), 12);
    }

    #[test]
    fn test_calculate_difference_with_zeros() {
        let array_1 = vec![0, 0, 0];
        let array_2 = vec![1, 2, 3];
        assert_eq!(calculate_difference(&array_1, &array_2), 6);
    }
}
