mod calculator;
mod input;
mod parser;

fn main() {
    let input = input::read_input();
    let pairs = parser::parse(input);
    let result = calculator::calculate(pairs);
    println!("{}", result);
}
