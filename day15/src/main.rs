extern crate derive_error;

mod number_stream;

use std::fs;

use number_stream::NumberStream;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let number_stream: NumberStream = input
        .parse()
        .expect("could not parse input as NumberStream");

    let nth = number_stream.nth(2020);
    println!("Stage 1: {}", nth);

    let nth = number_stream.nth(30000000);
    println!("Stage 2: {}", nth);
}
