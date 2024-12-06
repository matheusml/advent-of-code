mod bounds;
mod constants;
mod counter;
mod input;
mod pattern;

fn main() {
    let matrix = input::get_matrix();
    println!("{}", counter::count_matches(&matrix));
}
