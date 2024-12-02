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
