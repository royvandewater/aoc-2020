extern crate derive_error;

mod program;

use std::fs;

use program::Program;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let mut program: Program = input.parse().expect("could not parse input as Program");

    program.run();
    println!("Stage 1: {}", program.stage1_sum());
    println!("Stage 2: {}", program.stage2_sum());
}
