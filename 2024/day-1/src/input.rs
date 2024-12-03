pub fn read_input_arrays(file: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = std::fs::read_to_string(file).unwrap();
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        array1.push(numbers[0]);
        array2.push(numbers[1]);
    }

    array1.sort();
    array2.sort();

    (array1, array2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_valid_input() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "1 2\n3 4\n5 6").unwrap();

        let (array1, array2) = read_input_arrays(file_path.to_str().unwrap());
        assert_eq!(array1, vec![1, 3, 5]);
        assert_eq!(array2, vec![2, 4, 6]);
    }

    #[test]
    fn test_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let (array1, array2) = read_input_arrays(file_path.to_str().unwrap());
        assert!(array1.is_empty());
        assert!(array2.is_empty());
    }

    #[test]
    fn test_sorted_output() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "3 6\n1 4\n2 5").unwrap();

        let (array1, array2) = read_input_arrays(file_path.to_str().unwrap());
        assert_eq!(array1, vec![1, 2, 3]); // Should be sorted
        assert_eq!(array2, vec![4, 5, 6]); // Should be sorted
    }

    #[test]
    #[should_panic]
    fn test_invalid_number_format() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "1 2\n3 invalid\n5 6").unwrap();

        read_input_arrays(file_path.to_str().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_file_not_found() {
        read_input_arrays("nonexistent_file.txt");
    }
}
