extern crate derive_error;

mod error_rate;
mod input;

use std::fs;

use error_rate::error_rate;
use input::Input;

fn main() {
    let input: Input = fs::read_to_string("./input.txt")
        .expect("could not read ./input.txt")
        .parse()
        .expect("could not parse input as Input");

    println!(
        "Stage 1: {}",
        error_rate(&input.rules, &input.nearby_tickets)
    );
}
