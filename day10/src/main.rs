use std::fs;

use adapter_counter::AdapterCounter;

mod adapter_counter;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let adapter_counter: AdapterCounter = input.parse().expect("Could not parse adapters");

    let (ones, threes) = adapter_counter.get_counts();
    println!(
        "Ones: {}, Threes: {}, Answer: {}",
        ones,
        threes,
        ones * threes
    )
}
