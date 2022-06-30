use std::fs;

mod plane;
mod plane2;

use plane::Plane;
use plane2::Plane2;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let mut plane: Plane = input.parse().expect("Could not parse Plane");
    plane.stabilize();
    println!("Plane 1: Occupied seats: {}", plane.count_occupied_seats());

    let mut plane2: Plane2 = input.parse().expect("Could not parse Plane2");
    plane2.stabilize();
    println!("Plane 2: Occupied seats: {}", plane2.count_occupied_seats());
}
