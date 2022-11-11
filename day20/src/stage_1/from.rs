use crate::input::Input;

use super::{tile::Tile, Stage1};

impl From<&Input> for Stage1 {
    fn from(input: &Input) -> Self {
        Stage1 {
            tiles: input
                .iter()
                .map(|item| Tile { id: item.id, edges: get_edges(&item.tile) })
                .collect(),
        }
    }
}

fn get_edges(lines: &Vec<String>) -> [String; 4] {
    let north = lines.first().unwrap().to_string();
    let south = lines.last().unwrap().to_string();

    let east = lines.iter().map(|l| l.chars().next().unwrap()).collect();
    let west = lines.iter().map(|l| l.chars().last().unwrap()).collect();

    [north, east, south, west]
}
