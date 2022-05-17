use std::fs;
mod boarding;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let seats: Vec<boarding::Seat> = boarding::from_input(&input);

    println!("Max Seat: {:?}", seats.iter().max().unwrap());
    println!("Missing Seat: {:?}", boarding::find_missing(seats));
}
