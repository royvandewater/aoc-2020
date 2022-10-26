extern crate derive_error;

mod number_stream;

use std::fs;

use number_stream::NumberStream;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let number_stream: NumberStream = input
        .parse()
        .expect("could not parse input as NumberStream");

    let nth = number_stream.nth(2019); // stage 1 asks for the 2020th number, but we are 0 indexed
    println!("Stage 1: {}", nth);
}
