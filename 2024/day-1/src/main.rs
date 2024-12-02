mod input;

fn main() {
    let (array_1, array_2) = input::read_input_arrays("src/input.txt");

    let mut result: i32 = 0;

    for i in 0..array_1.len() {
        result += (array_2[i] - array_1[i]).abs();
    }

    println!("{}", result);
}
