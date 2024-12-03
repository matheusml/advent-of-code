mod calculator;
mod input;

fn main() {
    let (array_1, array_2) = input::read_input_arrays("src/input.txt");
    let result = calculator::calculate_difference(&array_1, &array_2);
    println!("{}", result);
}
