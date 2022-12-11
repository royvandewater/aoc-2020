extern crate derive_error;

mod input;
mod stage_1;
mod stage_2;

use std::fs;

use input::Input;
use stage_1::Stage1;
use stage_2::Stage2;

fn main() {
    let input_str = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let input: Input = input_str.parse().expect("could not parse input as Input");

    let stage_1_answer = Stage1::from(&input).answer();
    println!("Stage 1: {}", stage_1_answer);

    let stage_2_answer = Stage2::from(&input).answer().expect("Error computing answer for stage 2");
    println!("Stage 2: {}", stage_2_answer);
}
