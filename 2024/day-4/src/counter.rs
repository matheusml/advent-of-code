use crate::pattern::*;

pub fn count_matches(matrix: &Vec<Vec<char>>) -> i32 {
    let matchers = [
        match_right,
        match_left,
        match_up,
        match_down,
        match_up_right,
        match_up_left,
        match_down_right,
        match_down_left,
    ];

    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            count += matchers
                .iter()
                .filter(|&&matcher| matcher(matrix, (i, j)))
                .count() as i32;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matches() {
        let matrix = vec![
            vec!['.', '.', '.', '.', 'X', 'X', 'M', 'A', 'S', '.'],
            vec!['.', 'S', 'A', 'M', 'X', 'M', 'S', '.', '.', '.'],
            vec!['.', '.', '.', 'S', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', '.', 'M', 'S', '.', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', '.', 'M', 'M'],
            vec!['X', '.', '.', '.', '.', '.', 'X', 'A', '.', 'A'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', 'S'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', 'A'],
            vec!['.', '.', 'M', '.', 'M', '.', 'M', '.', 'M', 'M'],
            vec!['.', 'X', '.', 'X', '.', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(count_matches(&matrix), 18);
    }
}
