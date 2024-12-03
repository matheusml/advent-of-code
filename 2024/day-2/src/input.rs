use std::fs::read_to_string;

pub fn read_input() -> Vec<Vec<i32>> {
    let contents = read_to_string("src/input.txt").expect("Should have been able to read the file");

    contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_read_input() {
        // Create a temporary test file
        let test_content = "1 2 3\n4 5 6\n7 8 9";
        write("src/input.txt", test_content).expect("Failed to write test file");

        // Read and verify the content
        let result = read_input();
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(result, expected);
    }
}
