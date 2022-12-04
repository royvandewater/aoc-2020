use std::collections::{HashMap, HashSet};

use crate::input::Input;

use super::{
    tile::{canonize_edge, Tile},
    Stage2,
};

impl From<&Input> for Stage2 {
    fn from(input: &Input) -> Self {
        let tiles: Vec<Tile> = input.iter().map(Tile::from).collect();
        let shared_edges = find_shared_edges(&tiles);
        Stage2 { tiles, shared_edges }
    }
}

fn find_shared_edges(tiles: &Vec<Tile>) -> HashMap<String, HashSet<usize>> {
    let unique_edges = get_all_unique_edges(tiles);
    let mut shared_edges = unique_edges.clone();
    shared_edges.retain(|_, tiles| tiles.len() > 1);

    println!(
        "Tiles: {}, unique edges: {}, shared edges: {}",
        tiles.len(),
        unique_edges.len(),
        shared_edges.len(),
    );

    return shared_edges;
}

fn get_all_unique_edges(tiles: &Vec<Tile>) -> HashMap<String, HashSet<usize>> {
    let mut shared_edges: HashMap<String, HashSet<usize>> = HashMap::new();

    tiles.iter().for_each(|tile| {
        for edge in tile.possible_edges().iter() {
            let edge = canonize_edge(edge);
            let neighbors = shared_edges.entry(edge).or_default();
            neighbors.insert(tile.id);
        }
    });

    shared_edges
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::input::Input;

    use super::*;

    #[test]
    fn test_get_unique_edges() -> Result<(), String> {
        let input_str = fs::read_to_string("./example.txt").expect("could not read ./example.txt");
        let input: Input = input_str.parse().expect("could not parse input as Input");
        let tiles: Vec<Tile> = input.iter().map(Tile::from).collect();

        // A B C
        // D E F
        // G H I
        // 12 non-shared edges (border around the outside)
        // 12 shared edges (spaces between tiles)
        // 24 total edges

        let sut = get_all_unique_edges(&tiles);
        assert_eq!(24, sut.len());

        Ok(())
    }
}
