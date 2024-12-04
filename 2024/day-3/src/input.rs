use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("src/input.txt").expect("Should have been able to read the input file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input();
        assert!(!input.is_empty());
        assert!(input.contains("mul")); // Basic check that we got some expected content
    }
}
