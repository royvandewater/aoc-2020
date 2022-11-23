mod from;
mod tile;

use std::collections::{HashMap, HashSet};

use tile::Tile;

use self::tile::canonize_edge;

type Position = [usize; 2];

type UnplacedTiles = HashMap<usize, Tile>;
type Layout = HashMap<Position, Tile>;

pub struct Stage2 {
    tiles: Vec<Tile>,
    shared_edges: HashMap<String, HashSet<Tile>>,
}

impl Stage2 {
    pub fn answer(&self) -> Result<usize, String> {
        let (layout, unplaced_tiles) = self.initialize_layout()?;

        let _layout = self.solve(layout, unplaced_tiles)?;

        todo!()
    }

    fn initialize_layout(&self) -> Result<(Layout, UnplacedTiles), String> {
        let mut corner_tiles = self.corner_tiles();
        corner_tiles.sort();

        let mut unplaced_tiles: HashMap<usize, Tile> = self.tiles.iter().map(|t| (t.id, t.clone())).collect();
        let mut layout: HashMap<Position, Tile> = HashMap::new();
        let mut corner_tiles_iter = corner_tiles.iter();

        let shared_edges: HashSet<String> = self.shared_edges.keys().cloned().collect();
        // we need to rotate the first tile so that it's unmatched edges are down and to the left
        let first_tile = corner_tiles_iter
            .next()
            .ok_or("Couldn't retrieve first tile")?
            .rotate(&shared_edges, [true, true, false, false])?;

        layout.insert([0, 0], first_tile.clone());
        unplaced_tiles.remove(&first_tile.id);

        return Ok((layout, unplaced_tiles));
    }

    fn solve(
        &self,
        mut layout: HashMap<Position, Tile>,
        mut unplaced_tiles: HashMap<usize, Tile>,
    ) -> Result<HashMap<Position, Tile>, String> {
        if unplaced_tiles.is_empty() {
            return Ok(layout);
        }

        let start_count = unplaced_tiles.len();

        let mut positions: Vec<Position> = layout.keys().cloned().collect();
        positions.sort();

        for position in positions.iter() {
            println!(
                "tile: {}, solving position: {:?}",
                layout.get(position).unwrap().clone().id,
                position
            );
            (layout, unplaced_tiles) = self.populate_neighbors(layout, unplaced_tiles, position);
        }

        if start_count == unplaced_tiles.len() {
            return Err(format!("Failed to place any tiles with {} tiles left", start_count));
        }

        self.solve(layout, unplaced_tiles)
    }

    fn populate_neighbors(
        &self,
        mut layout: Layout,
        mut unplaced_tiles: UnplacedTiles,
        position: &Position,
    ) -> (Layout, UnplacedTiles) {
        let tile = layout.get(position).unwrap().clone();

        let before = layout.len();
        (layout, unplaced_tiles) = self.populate_north_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a north tile")
        }
        let before = layout.len();
        (layout, unplaced_tiles) = self.populate_east_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found an east tile")
        }
        let before = layout.len();
        (layout, unplaced_tiles) = self.populate_south_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a south tile")
        }
        let before = layout.len();
        (layout, unplaced_tiles) = self.populate_west_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a west tile")
        }

        return (layout, unplaced_tiles);
    }

    fn populate_north_neighbor(
        &self,
        layout: Layout,
        unplaced_tiles: UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) -> (Layout, UnplacedTiles) {
        let &[x, y] = position;

        let north_position: Position = [x, y + 1];
        let north_edge = &tile.edges[0];

        self.replace_with_tile_matching_edge(layout, unplaced_tiles, tile, north_position, north_edge)
    }

    fn populate_east_neighbor(
        &self,
        layout: Layout,
        unplaced_tiles: UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) -> (Layout, UnplacedTiles) {
        let &[x, y] = position;

        let east_position: Position = [x + 1, y];
        let east_edge = &tile.edges[1];

        self.replace_with_tile_matching_edge(layout, unplaced_tiles, tile, east_position, east_edge)
    }

    fn populate_south_neighbor(
        &self,
        layout: Layout,
        unplaced_tiles: UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) -> (Layout, UnplacedTiles) {
        let &[x, y] = position;

        if y == 0 {
            return (layout, unplaced_tiles);
        }

        let south_position: Position = [x, y - 1];
        let south_edge = &tile.edges[2];

        self.replace_with_tile_matching_edge(layout, unplaced_tiles, tile, south_position, south_edge)
    }

    fn populate_west_neighbor(
        &self,
        layout: Layout,
        unplaced_tiles: UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) -> (Layout, UnplacedTiles) {
        let &[x, y] = position;

        if x == 0 {
            return (layout, unplaced_tiles);
        }

        let west_position: Position = [x - 1, y];
        let west_edge = &tile.edges[3];

        self.replace_with_tile_matching_edge(layout, unplaced_tiles, tile, west_position, west_edge)
    }

    fn replace_with_tile_matching_edge(
        &self,
        mut layout: Layout,
        mut unplaced_tiles: UnplacedTiles,
        tile: &Tile,
        position: Position,
        edge: &String,
    ) -> (Layout, UnplacedTiles) {
        if layout.contains_key(&position) {
            return (layout, unplaced_tiles);
        }

        let reverse_edge: String = edge.chars().rev().collect();

        match self.shared_edges.get(edge).or(self.shared_edges.get(&reverse_edge)) {
            None => return (layout, unplaced_tiles),
            Some(neighbors) => {
                let mut neighbors = neighbors.clone();
                neighbors.remove(tile);
                let north_tile = neighbors.iter().next().unwrap().clone();

                unplaced_tiles.remove(&north_tile.id);
                layout.insert(position, north_tile);

                return (layout, unplaced_tiles);
            }
        }
    }

    fn corner_tiles(&self) -> Vec<Tile> {
        self.tiles
            .iter()
            .filter(|&t| edge_count_is_2(&self.shared_edges, t))
            .cloned()
            .collect()
    }
}

fn edge_count_is_2(shared_edges: &HashMap<String, HashSet<Tile>>, tile: &Tile) -> bool {
    let shared_edge_count = tile
        .edges
        .iter()
        .filter(|&e| {
            let edge = canonize_edge(e);
            shared_edges.contains_key(&edge)
        })
        .count();

    shared_edge_count == 2
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::input::Input;

    use super::*;

    #[test]
    fn test_example() -> Result<(), String> {
        let input_str = fs::read_to_string("./example.txt").expect("could not read ./example.txt");
        let input: Input = input_str.parse().expect("could not parse input as Input");

        let sut = Stage2::from(&input);
        assert_eq!(273, sut.answer()?);

        Ok(())
    }
}
