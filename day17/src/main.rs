extern crate derive_error;

mod process_cycles;
mod state;

use std::fs;

use process_cycles::process_cycles;
use state::State;

use crate::process_cycles::count_active_cubes;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let begin_state: State = input.parse().expect("could not parse input as State");

    let end_state: State = process_cycles(6, &begin_state);
    let end_count: usize = count_active_cubes(end_state);
    println!("Stage 1: {}", end_count);
}
