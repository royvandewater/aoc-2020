mod invalid_number_finder;
mod weakness_finder;

use std::fs;

use invalid_number_finder::InvalidNumberFinder;
use weakness_finder::WeaknessFinder;

fn main() {
    let full_input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let preamble: Vec<&str> = full_input.lines().take(25).collect();
    let input: Vec<&str> = full_input.lines().skip(25).collect();

    let mut invalid_number_finder = match InvalidNumberFinder::try_from((preamble, input)) {
        Err(e) => panic!("Error constructing InvalidNumberFinder: {}", e),
        Ok(s) => s,
    };

    let first_invalid_number = invalid_number_finder.first_invalid_number();
    println!("First invalid number is: {}", first_invalid_number);

    let full_input_lines: Vec<&str> = full_input.lines().collect();
    let weakness_finder = match WeaknessFinder::try_from(full_input_lines) {
        Err(e) => panic!("Error constructing WeaknessFinder: {}", e),
        Ok(s) => s,
    };

    match weakness_finder.find_weakness(first_invalid_number) {
        Err(e) => panic!("Error finding weakness: {}", e),
        Ok(weakness) => println!("Weakness: {}", weakness),
    }
}
