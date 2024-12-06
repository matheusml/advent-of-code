use crate::bounds::*;

pub fn match_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_right(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0][position.1 + 1] == 'M'
        && matrix[position.0][position.1 + 2] == 'A'
        && matrix[position.0][position.1 + 3] == 'S'
}

pub fn match_left(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_left(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0][position.1 - 1] == 'M'
        && matrix[position.0][position.1 - 2] == 'A'
        && matrix[position.0][position.1 - 3] == 'S'
}

pub fn match_up(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_up(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 - 1][position.1] == 'M'
        && matrix[position.0 - 2][position.1] == 'A'
        && matrix[position.0 - 3][position.1] == 'S'
}

pub fn match_down(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_down(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 + 1][position.1] == 'M'
        && matrix[position.0 + 2][position.1] == 'A'
        && matrix[position.0 + 3][position.1] == 'S'
}

pub fn match_up_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_up_right(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 - 1][position.1 + 1] == 'M'
        && matrix[position.0 - 2][position.1 + 2] == 'A'
        && matrix[position.0 - 3][position.1 + 3] == 'S'
}

pub fn match_up_left(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_up_left(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 - 1][position.1 - 1] == 'M'
        && matrix[position.0 - 2][position.1 - 2] == 'A'
        && matrix[position.0 - 3][position.1 - 3] == 'S'
}

pub fn match_down_right(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_down_right(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 + 1][position.1 + 1] == 'M'
        && matrix[position.0 + 2][position.1 + 2] == 'A'
        && matrix[position.0 + 3][position.1 + 3] == 'S'
}

pub fn match_down_left(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> bool {
    can_go_down_left(matrix, position)
        && matrix[position.0][position.1] == 'X'
        && matrix[position.0 + 1][position.1 - 1] == 'M'
        && matrix[position.0 + 2][position.1 - 2] == 'A'
        && matrix[position.0 + 3][position.1 - 3] == 'S'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_right() {
        let matrix = vec![vec!['T', 'X', 'M', 'A', 'S', 'X']];
        assert!(!match_right(&matrix, (0, 0)));
        assert!(match_right(&matrix, (0, 1)));
        assert!(!match_right(&matrix, (0, 2)));
        assert!(!match_right(&matrix, (0, 3)));
        assert!(!match_right(&matrix, (0, 4)));
        assert!(!match_right(&matrix, (0, 5)));
    }

    #[test]
    fn test_match_left() {
        let matrix = vec![vec!['Z', 'X', 'S', 'A', 'M', 'X', 'A']];
        assert!(!match_left(&matrix, (0, 6)));
        assert!(match_left(&matrix, (0, 5)));
        assert!(!match_left(&matrix, (0, 4)));
        assert!(!match_left(&matrix, (0, 3)));
        assert!(!match_left(&matrix, (0, 2)));
        assert!(!match_left(&matrix, (0, 1)));
        assert!(!match_left(&matrix, (0, 0)));
    }

    #[test]
    fn test_match_up() {
        let matrix = vec![
            vec!['X', 'S', 'A', 'S'],
            vec!['X', 'A', 'A', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'X', 'A', 'S'],
        ];
        assert!(!match_up(&matrix, (0, 0)));
        assert!(!match_up(&matrix, (0, 1)));
        assert!(!match_up(&matrix, (0, 2)));
        assert!(!match_up(&matrix, (0, 3)));
        assert!(!match_up(&matrix, (1, 0)));
        assert!(!match_up(&matrix, (1, 1)));
        assert!(!match_up(&matrix, (1, 2)));
        assert!(!match_up(&matrix, (1, 3)));
        assert!(!match_up(&matrix, (2, 0)));
        assert!(!match_up(&matrix, (2, 1)));
        assert!(!match_up(&matrix, (2, 2)));
        assert!(!match_up(&matrix, (2, 3)));
        assert!(!match_up(&matrix, (3, 0)));
        assert!(match_up(&matrix, (3, 1)));
        assert!(!match_up(&matrix, (3, 2)));
        assert!(!match_up(&matrix, (3, 3)));
    }

    #[test]
    fn test_match_down() {
        let matrix = vec![
            vec!['X', 'S', 'X', 'S'],
            vec!['X', 'A', 'M', 'S'],
            vec!['X', 'M', 'A', 'S'],
            vec!['X', 'X', 'S', 'S'],
        ];
        assert!(!match_down(&matrix, (0, 0)));
        assert!(!match_down(&matrix, (0, 1)));
        assert!(match_down(&matrix, (0, 2)));
        assert!(!match_down(&matrix, (0, 3)));
        assert!(!match_down(&matrix, (1, 0)));
        assert!(!match_down(&matrix, (1, 1)));
        assert!(!match_down(&matrix, (1, 2)));
        assert!(!match_down(&matrix, (1, 3)));
        assert!(!match_down(&matrix, (2, 0)));
        assert!(!match_down(&matrix, (2, 1)));
        assert!(!match_down(&matrix, (2, 2)));
        assert!(!match_down(&matrix, (2, 3)));
        assert!(!match_down(&matrix, (3, 0)));
        assert!(!match_down(&matrix, (3, 1)));
        assert!(!match_down(&matrix, (3, 2)));
        assert!(!match_down(&matrix, (3, 3)));
    }

    #[test]
    fn test_match_up_right() {
        let matrix = vec![
            vec!['S', 'X', 'X', 'S'],
            vec!['A', 'M', 'A', 'S'],
            vec!['M', 'M', 'A', 'S'],
            vec!['X', 'X', 'M', 'S'],
        ];
        assert!(!match_up_right(&matrix, (0, 0)));
        assert!(!match_up_right(&matrix, (1, 0)));
        assert!(!match_up_right(&matrix, (2, 0)));
        assert!(match_up_right(&matrix, (3, 0)));
        assert!(!match_up_right(&matrix, (3, 1)));
    }

    #[test]
    fn test_match_up_left() {
        let matrix = vec![
            vec!['S', 'X', 'X', 'S'],
            vec!['A', 'A', 'X', 'S'],
            vec!['M', 'X', 'M', 'S'],
            vec!['S', 'M', 'X', 'X'],
        ];
        assert!(!match_up_left(&matrix, (0, 3)));
        assert!(!match_up_left(&matrix, (1, 3)));
        assert!(!match_up_left(&matrix, (2, 3)));
        assert!(match_up_left(&matrix, (3, 3)));
        assert!(!match_up_left(&matrix, (3, 2)));
    }

    #[test]
    fn test_match_down_right() {
        let matrix = vec![
            vec!['X', 'X', 'M', 'S'],
            vec!['S', 'M', 'A', 'S'],
            vec!['M', 'A', 'A', 'S'],
            vec!['S', 'S', 'M', 'S'],
        ];
        assert!(match_down_right(&matrix, (0, 0)));
        assert!(!match_down_right(&matrix, (0, 1)));
        assert!(!match_down_right(&matrix, (1, 0)));
        assert!(!match_down_right(&matrix, (2, 0)));
        assert!(!match_down_right(&matrix, (3, 0)));
    }

    #[test]
    fn test_match_down_left() {
        let matrix = vec![
            vec!['S', 'M', 'A', 'X'],
            vec!['M', 'A', 'M', 'S'],
            vec!['A', 'A', 'S', 'S'],
            vec!['S', 'S', 'M', 'S'],
        ];
        assert!(match_down_left(&matrix, (0, 3)));
        assert!(!match_down_left(&matrix, (0, 2)));
        assert!(!match_down_left(&matrix, (1, 3)));
        assert!(!match_down_left(&matrix, (2, 3)));
        assert!(!match_down_left(&matrix, (3, 3)));
    }
}
