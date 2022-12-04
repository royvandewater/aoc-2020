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
    shared_edges: HashMap<String, HashSet<usize>>,
}

impl Stage2 {
    pub fn answer(&self) -> Result<usize, String> {
        let (mut layout, mut unplaced_tiles) = self.initialize_layout()?;

        self.solve(&mut layout, &mut unplaced_tiles)?;

        todo!()
    }

    fn initialize_layout(&self) -> Result<(Layout, UnplacedTiles), String> {
        let mut corner_tiles = self.corner_tiles();
        corner_tiles.sort();

        let mut unplaced_tiles: HashMap<usize, Tile> = self.tiles.iter().map(|t| (t.id, t.clone())).collect();
        let mut layout: HashMap<Position, Tile> = HashMap::new();
        let mut corner_tiles_iter = corner_tiles.iter();

        let shared_edges: HashSet<String> = self.shared_edges.keys().cloned().collect();
        // we need to rotate the first tile so that it's unmatched edges are up and to the left
        let first_tile = corner_tiles_iter
            .next()
            .ok_or("Couldn't retrieve first tile")?
            .rotate_matched(&shared_edges, [false, true, true, false])?;

        layout.insert([0, 0], first_tile.clone());
        unplaced_tiles.remove(&first_tile.id);

        return Ok((layout, unplaced_tiles));
    }

    fn solve(&self, layout: &mut Layout, unplaced_tiles: &mut UnplacedTiles) -> Result<(), String> {
        println!("==============");
        print_layout(&layout);
        println!("==============");

        if unplaced_tiles.is_empty() {
            return Ok(());
        }

        let start_count = unplaced_tiles.len();
        self.step(layout, unplaced_tiles);

        if start_count == unplaced_tiles.len() {
            return Err(format!("Failed to place any tiles with {} tiles left", start_count));
        }

        self.solve(layout, unplaced_tiles)
    }

    fn step(&self, layout: &mut Layout, unplaced_tiles: &mut UnplacedTiles) {
        let mut positions: Vec<Position> = layout.keys().cloned().collect();
        positions.sort();

        for position in positions.iter() {
            println!(
                "tile: {}, solving position: {:?}",
                layout.get(position).unwrap().clone().id,
                position
            );
            self.populate_neighbors(layout, unplaced_tiles, position);
        }
    }

    fn populate_neighbors(&self, layout: &mut Layout, unplaced_tiles: &mut UnplacedTiles, position: &Position) {
        let tile = layout.get(position).unwrap().clone();

        let before = layout.len();
        self.populate_north_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a north tile");
        }
        let before = layout.len();
        self.populate_east_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found an east tile");
        }
        let before = layout.len();
        self.populate_south_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a south tile");
        }
        let before = layout.len();
        self.populate_west_neighbor(layout, unplaced_tiles, position, &tile);
        let after = layout.len();
        if before != after {
            println!("Found a west tile");
        }
    }

    fn populate_north_neighbor(
        &self,
        layout: &mut Layout,
        unplaced_tiles: &mut UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) {
        let &[x, y] = position;

        if y == 0 {
            return;
        }

        let north_position: Position = [x, y - 1];
        let north_edge = &tile.edges[0];
        println!("north of tile: {} is edge: {:?}", tile.id, north_edge);

        self.insert_tile_matching_edge(layout, unplaced_tiles, tile, north_position, north_edge)
    }

    fn populate_east_neighbor(
        &self,
        layout: &mut Layout,
        unplaced_tiles: &mut UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) {
        let &[x, y] = position;

        let east_position: Position = [x + 1, y];
        let east_edge = &tile.edges[1];
        println!("east of tile: {} is edge: {:?}", tile.id, east_edge);

        self.insert_tile_matching_edge(layout, unplaced_tiles, tile, east_position, east_edge)
    }

    fn populate_south_neighbor(
        &self,
        layout: &mut Layout,
        unplaced_tiles: &mut UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) {
        let &[x, y] = position;

        let south_position: Position = [x, y + 1];
        let south_edge = &tile.edges[2];
        println!("south of tile: {} is edge: {:?}", tile.id, south_edge);

        self.insert_tile_matching_edge(layout, unplaced_tiles, tile, south_position, south_edge)
    }

    fn populate_west_neighbor(
        &self,
        layout: &mut Layout,
        unplaced_tiles: &mut UnplacedTiles,
        position: &Position,
        tile: &Tile,
    ) {
        let &[x, y] = position;

        if x == 0 {
            return;
        }

        let west_position: Position = [x - 1, y];
        let west_edge = &tile.edges[3];
        println!("west of tile: {} is edge: {:?}", tile.id, west_edge);

        self.insert_tile_matching_edge(layout, unplaced_tiles, tile, west_position, west_edge)
    }

    fn insert_tile_matching_edge(
        &self,
        layout: &mut Layout,
        unplaced_tiles: &mut UnplacedTiles,
        tile: &Tile,
        position: Position,
        edge: &String,
    ) {
        if layout.contains_key(&position) {
            return;
        }

        let edge = canonize_edge(edge);

        match self.shared_edges.get(&edge) {
            None => return,
            Some(neighbors) => {
                let mut neighbors = neighbors.clone();
                let removed = neighbors.remove(&tile.id);

                if !removed {
                    panic!(
                        "The current tile was not a valid neighbor for this edge: {}, {:?}",
                        edge, neighbors
                    );
                }

                let tile_id_to_insert = neighbors.iter().next().unwrap().clone();
                let mut tile_to_insert = unplaced_tiles.remove(&tile_id_to_insert).unwrap_or_else(|| {
                    println!("XXXXXXXXXXXXXX");
                    print_layout(layout);
                    println!("XXXXXXXXXXXXXX");

                    panic!(
                        "Tile with id {} was not in unplaced_tiles: {:?}\nPosition: {:?}\nLayout: {:?}",
                        tile_id_to_insert,
                        unplaced_tiles.keys(),
                        position,
                        layout.iter().map(|(p, t)| (p, t.id)).collect::<Vec<(&Position, usize)>>(),
                    )
                });
                self.flip_new_tile_until_it_faces_old_tile(&edge, tile, &mut tile_to_insert);
                self.rotate_new_tile_until_it_faces_old_tile(&edge, tile, &mut tile_to_insert);

                layout.insert(position, tile_to_insert);
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

    fn rotate_new_tile_until_it_faces_old_tile(&self, edge: &String, tile: &Tile, new_tile: &mut Tile) {
        let edge = canonize_edge(edge);

        let direction_edge_faces_on_old = tile
            .edges
            .iter()
            .enumerate()
            .find_map(|(i, e)| match canonize_edge(e) == edge {
                true => Some(i),
                false => None,
            })
            .unwrap();

        let direction_edge_should_face_on_new = (direction_edge_faces_on_old + 2) % 4;

        for _ in 0..4 {
            new_tile.rotate_once();

            let adjacent_edge = canonize_edge(&new_tile.edges[direction_edge_should_face_on_new]);

            if edge == adjacent_edge {
                return;
            }
        }

        panic!("Failed to find matching rotation");
    }

    fn flip_new_tile_until_it_faces_old_tile(&self, edge: &String, tile: &Tile, new_tile: &mut Tile) {
        let c_edge = canonize_edge(edge);

        let existing_edge = tile.edges.iter().find(|e| canonize_edge(e) == c_edge).unwrap();
        let new_edge = new_tile.edges.iter().find(|e| canonize_edge(e) == c_edge).unwrap();

        let target_edge: String = existing_edge.chars().rev().collect();

        if new_edge == &target_edge {
            return;
        }

        let direction_edge_faces_on_new = new_tile
            .edges
            .iter()
            .enumerate()
            .find_map(|(i, e)| match canonize_edge(e) == c_edge {
                true => Some(i),
                false => None,
            })
            .unwrap();

        match direction_edge_faces_on_new {
            0 | 2 => new_tile.flip_horizontally(),
            1 | 3 => new_tile.flip_vertically(),
            x => panic!("Unknown direction: {}", x),
        };
    }
}

fn edge_count_is_2(shared_edges: &HashMap<String, HashSet<usize>>, tile: &Tile) -> bool {
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

fn layout_to_str(layout: &Layout) -> String {
    let mut output: Vec<String> = Vec::new();

    let normalized = normalize_layout(layout);

    let &max_x = normalized.iter().map(|([x, _y], _)| x).max().unwrap_or(&0);
    let &max_y = normalized.iter().map(|([_x, y], _)| y).max().unwrap_or(&0);

    for y in 0..=max_y {
        let mut line: Vec<char> = Vec::new();

        for x in 0..=max_x {
            let &c = normalized.get(&[x, y]).unwrap_or(&' ');
            line.push(c);
        }

        output.push(line.iter().collect::<String>().trim().into());
    }

    return output.join("\n");
}

fn print_layout(layout: &Layout) {
    println!("{}", layout_to_str(layout));
}

fn normalize_layout(layout: &Layout) -> HashMap<Position, char> {
    let &max_x = layout.iter().map(|([x, _y], _)| x).max().unwrap_or(&0);
    let &max_y = layout.iter().map(|([_x, y], _)| y).max().unwrap_or(&0);

    let mut everything: HashMap<Position, char> = HashMap::new();

    let tile_size = 1 + layout.values().next().unwrap().edges[0].len();

    for y in 0..=max_y {
        for x in 0..=max_x {
            if let Some(tile) = layout.get(&[x, y]) {
                for (n, line) in tile.lines.iter().enumerate() {
                    for (m, c) in line.chars().enumerate() {
                        let tile_x = m + (x * tile_size);
                        let tile_y = n + (y * tile_size);

                        everything.insert([tile_x, tile_y], c);
                    }
                }
            }
        }
    }

    return everything;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::input::Input;

    use super::*;

    #[test]
    fn test_example_1() -> Result<(), String> {
        let input_str = fs::read_to_string("./example.txt").expect("could not read ./example.txt");
        let input: Input = input_str.parse().expect("could not parse input as Input");

        let sut = Stage2::from(&input);
        assert_eq!(273, sut.answer()?);

        Ok(())
    }

    #[test]
    fn test_initialize_layout_simple() {
        let input_str = "
            Tile 1:
            1#
            .#

            Tile 2:
            #2
            #x

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (layout, _) = sut.initialize_layout().expect("could not initialize layout");

        let expected = "
            1#
            .#
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }

    #[test]
    fn test_step_simple() {
        let input_str = "
            Tile 1:
            1#
            .#

            Tile 2:
            #2
            #x

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (mut layout, mut unplaced_tiles) = sut.initialize_layout().expect("could not initialize layout");

        sut.step(&mut layout, &mut unplaced_tiles);

        let expected = "
            1# #2
            .# #x

            .#
            3=
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }

    #[test]
    fn test_solve_simple() {
        let input_str = "
            Tile 1:
            1#
            .#

            Tile 2:
            #2
            #x

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (mut layout, mut unplaced_tiles) = sut.initialize_layout().expect("could not initialize layout");

        sut.solve(&mut layout, &mut unplaced_tiles).expect("could not solve layout");

        let expected = "
            1# #2
            .# #x

            .# #x
            3= =4
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }

    #[test]
    fn test_solve_simple_rotation() {
        let input_str = "
            Tile 1:
            1#
            .#

            Tile 2:
            2x
            ##

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (mut layout, mut unplaced_tiles) = sut.initialize_layout().expect("could not initialize layout");

        sut.solve(&mut layout, &mut unplaced_tiles).expect("could not solve layout");

        let expected = "
            1# #2
            .# #x

            .# #x
            3= =4
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }

    #[test]
    fn test_solve_simple_flip_vertically() {
        let input_str = "
            Tile 1:
            1$
            .#

            Tile 2:
            #x
            $2

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (mut layout, mut unplaced_tiles) = sut.initialize_layout().expect("could not initialize layout");

        sut.solve(&mut layout, &mut unplaced_tiles).expect("could not solve layout");

        let expected = "
            1$ $2
            .# #x

            .# #x
            3= =4
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }

    #[test]
    fn test_solve_simple_flip_horizontally() {
        let input_str = "
            Tile 1:
            1$
            .#

            Tile 2:
            2$
            x#

            Tile 3:
            .#
            3=

            Tile 4:
            #x
            =4
        ";

        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut = Stage2::from(&input);
        let (mut layout, mut unplaced_tiles) = sut.initialize_layout().expect("could not initialize layout");

        sut.solve(&mut layout, &mut unplaced_tiles).expect("could not solve layout");

        let expected = "
            1$ $2
            .# #x

            .# #x
            3= =4
        "
        .trim()
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join("\n");

        let actual = layout_to_str(&layout);

        println!("\nexpected: \n{}", expected);
        println!("\nactual: \n{}", actual);

        assert_eq!(expected, layout_to_str(&layout))
    }
}
