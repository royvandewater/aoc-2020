use std::fs;

mod instruction;
mod ship;
mod waypoint_ship;

use ship::Ship;
use waypoint_ship::WaypointShip;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let mut ship: Ship = input.parse().expect("Could not parse Ship");
    ship.run();
    println!(
        "Manhattan Distance: {}",
        ship.manhattan_distance_from_origin()
    );

    let mut waypoint_ship: WaypointShip = input.parse().expect("Could not parse Waypoint");
    waypoint_ship.run();
    println!(
        "Manhattan Distance: {}",
        waypoint_ship.manhattan_distance_from_origin()
    )
}
