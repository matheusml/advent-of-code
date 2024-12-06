use crate::constants::XMAS_LENGTH;

pub fn can_go_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.1 < matrix[0].len() - (XMAS_LENGTH - 1)
}

pub fn can_go_left(_matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.1 >= XMAS_LENGTH - 1
}

pub fn can_go_up(_matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 >= XMAS_LENGTH - 1
}

pub fn can_go_down(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 < matrix.len() - (XMAS_LENGTH - 1)
}

pub fn can_go_up_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 >= XMAS_LENGTH - 1 && position.1 <= matrix[0].len() - XMAS_LENGTH
}

pub fn can_go_up_left(_matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 >= XMAS_LENGTH - 1 && position.1 >= XMAS_LENGTH - 1
}

pub fn can_go_down_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 <= matrix.len() - XMAS_LENGTH && position.1 <= matrix[0].len() - XMAS_LENGTH
}

pub fn can_go_down_left(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    position.0 <= matrix.len() - XMAS_LENGTH && position.1 >= XMAS_LENGTH - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_go_right() {
        let matrix = vec![vec!['X', 'M', 'A', 'S']];
        assert!(can_go_right(&matrix, (0, 0)));
        assert!(!can_go_right(&matrix, (0, 1)));
        assert!(!can_go_right(&matrix, (0, 2)));
        assert!(!can_go_right(&matrix, (0, 3)));
    }

    #[test]
    fn test_can_go_left() {
        let matrix = vec![vec!['X', 'M', 'A', 'S']];
        assert!(can_go_left(&matrix, (0, 3)));
        assert!(!can_go_left(&matrix, (0, 2)));
        assert!(!can_go_left(&matrix, (0, 1)));
        assert!(!can_go_left(&matrix, (0, 0)));
    }

    #[test]
    fn test_can_go_up() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(!can_go_up(&matrix, (0, 0)));
        assert!(!can_go_up(&matrix, (1, 0)));
        assert!(!can_go_up(&matrix, (2, 0)));
        assert!(can_go_up(&matrix, (3, 0)));
    }

    #[test]
    fn test_can_go_down() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(can_go_down(&matrix, (0, 0)));
        assert!(!can_go_down(&matrix, (1, 0)));
        assert!(!can_go_down(&matrix, (2, 0)));
        assert!(!can_go_down(&matrix, (3, 0)));
    }

    #[test]
    fn test_can_go_up_right() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(!can_go_up_right(&matrix, (0, 0)));
        assert!(!can_go_up_right(&matrix, (1, 0)));
        assert!(!can_go_up_right(&matrix, (2, 0)));
        assert!(!can_go_up_right(&matrix, (3, 3)));
        assert!(can_go_up_right(&matrix, (3, 0)));
    }

    #[test]
    fn test_can_go_up_left() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(!can_go_up_left(&matrix, (0, 0)));
        assert!(!can_go_up_left(&matrix, (1, 1)));
        assert!(!can_go_up_left(&matrix, (2, 2)));
        assert!(can_go_up_left(&matrix, (3, 3)));
    }

    #[test]
    fn test_can_go_down_right() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(can_go_down_right(&matrix, (0, 0)));
        assert!(!can_go_down_right(&matrix, (1, 1)));
        assert!(!can_go_down_right(&matrix, (2, 2)));
        assert!(!can_go_down_right(&matrix, (3, 3)));
    }

    #[test]
    fn test_can_go_down_left() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
        ];
        assert!(!can_go_down_left(&matrix, (0, 0)));
        assert!(!can_go_down_left(&matrix, (1, 1)));
        assert!(can_go_down_left(&matrix, (0, 3)));
        assert!(!can_go_down_left(&matrix, (3, 3)));
    }
}
