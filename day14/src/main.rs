extern crate derive_error;

mod program;

use std::fs;

use program::Program;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let mut program: Program = input.parse().expect("could not parse input as Program");

    program.run();
    println!("Sum: {}", program.sequence_sum());
}
