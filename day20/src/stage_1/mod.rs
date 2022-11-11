mod from;
mod tile;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use tile::Tile;

pub struct Stage1 {
    tiles: Vec<Tile>,

    layout: RefCell<HashMap<[isize; 2], HashSet<Tile>>>,
    initialized: RefCell<bool>,
}

impl Stage1 {
    pub fn answer(&self) -> Result<usize, String> {
        self.initialize()?;
        self.solve()?;
        let product = self.tiles.iter().map(|t| t.id).product();

        Ok(product)
    }

    fn dimensions(&self) -> isize {
        (self.tiles.len() as f32).sqrt() as isize
    }

    fn initialize(&self) -> Result<(), String> {
        if *self.initialized.borrow() {
            return Ok(());
        }

        let dimensions = self.dimensions();

        let mut tiles = self.tiles.iter();
        let first = tiles.next().ok_or("Could not grab fist tile")?;
        let rest: HashSet<Tile> = tiles.cloned().collect();

        let mut layout = self.layout.borrow_mut();

        for x in -dimensions..=dimensions {
            for y in -dimensions..=dimensions {
                layout.insert([x, y], rest.clone());
            }
        }

        layout.insert([0, 0], HashSet::from([first.clone()]));

        self.initialized.replace(true);
        Ok(())
    }

    fn solve(&self) -> Result<(), String> {
        self.print_known_tiles();
        if self.is_solved() {
            return Ok(());
        }

        let reduced_layout = reduce_layout(self.layout.borrow())?;
        let old_entropy = total_entropy(&self.layout.borrow());
        let new_entropy = total_entropy(&reduced_layout);

        println!("{} -> {}", old_entropy, new_entropy);

        if old_entropy <= new_entropy {
            return Err(format!(
                "Failed to reduce entropy: {} == {}",
                old_entropy, new_entropy
            ));
        }

        if new_entropy == 0 {
            return Err(format!(
                "0 entropy detected: {}, {}",
                old_entropy, new_entropy
            ));
        }

        self.layout.replace(reduced_layout);
        self.solve()
    }

    fn is_solved(&self) -> bool {
        let dimensions = self.dimensions();

        for x in -dimensions..=0 {
            for y in -dimensions..=0 {
                if self.solved_square_at(&[x, y], dimensions) {
                    return true;
                }
            }
        }

        return false;
    }

    fn solved_square_at(&self, position: &[isize; 2], dimensions: isize) -> bool {
        let &[startx, starty] = position;
        let [endx, endy] = [startx + dimensions, starty + dimensions];
        let layout = self.layout.borrow();

        for x in startx..endx {
            for y in starty..endy {
                if 1 != layout.get(&[x, y]).unwrap().len() {
                    return false;
                }
            }
        }

        return true;
    }

    fn print_known_tiles(&self) {
        let dimensions = self.dimensions();
        let layout = self.layout.borrow();

        print!("\n");
        for x in -dimensions..=dimensions {
            for y in -dimensions..=dimensions {
                let options = layout.get(&[x, y]).unwrap();
                match options.len() {
                    x if x < 10 => print!("{}", x),
                    _ => print!("."),
                }
            }
            print!("\n");
        }
    }
}

fn reduce_layout(
    layout: std::cell::Ref<HashMap<[isize; 2], HashSet<Tile>>>,
) -> Result<HashMap<[isize; 2], HashSet<Tile>>, String> {
    Ok(layout
        .iter()
        .map(|(position, tiles)| {
            (
                position.clone(),
                reduce_tiles_for_position(&layout, position, tiles),
            )
        })
        .collect())
}

fn reduce_tiles_for_position(
    layout: &std::cell::Ref<HashMap<[isize; 2], HashSet<Tile>>>,
    position: &[isize; 2],
    tiles: &HashSet<Tile>,
) -> HashSet<Tile> {
    tiles
        .iter()
        .filter(|tile| is_valid_tile(&layout, position, tile))
        .cloned()
        .collect()
}

fn is_valid_tile(
    layout: &HashMap<[isize; 2], HashSet<Tile>>,
    &[x, y]: &[isize; 2],
    tile: &Tile,
) -> bool {
    if x == 1 && y == 0 && tile.id == 2 {
        println!("Debugger");
    }

    let mut valid_edges = 0;

    let north = [x, y + 1];
    if let Some(other) = layout.get(&north) {
        if other.iter().any(|other| tile.edges[0] == other.edges[2]) {
            valid_edges += 1;
        }
    }

    let east = [x + 1, y];
    if let Some(other) = layout.get(&east) {
        if other.iter().any(|other| tile.edges[1] == other.edges[3]) {
            valid_edges += 1;
        }
    }

    let south = [x, y - 1];
    if let Some(other) = layout.get(&south) {
        if other.iter().any(|other| tile.edges[2] == other.edges[0]) {
            valid_edges += 1;
        }
    }

    let west = [x - 1, y];
    if let Some(other) = layout.get(&west) {
        if other.iter().any(|other| tile.edges[3] == other.edges[1]) {
            valid_edges += 1;
        }
    }

    return valid_edges >= 2;
}

fn total_entropy(layout: &HashMap<[isize; 2], HashSet<Tile>>) -> usize {
    layout.iter().map(|(_, tile)| tile.len()).sum()
}

impl Default for Stage1 {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            layout: RefCell::new(HashMap::new()),
            initialized: RefCell::new(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() -> Result<(), String> {
        let sut: Stage1 = Stage1 {
            // layout:
            // 1 <0>  2
            // ^      ^
            // 1      2
            // v      v
            // 4 <3>  5
            tiles: vec![
                Tile { id: 1, edges: ["n.".into(), "..".into(), ".#".into(), "w.".into()] },
                Tile { id: 2, edges: ["n.".into(), "e.".into(), "#.".into(), "..".into()] },
                Tile { id: 3, edges: [".#".into(), "##".into(), "s.".into(), "w.".into()] },
                Tile { id: 4, edges: ["#.".into(), "e.".into(), "s.".into(), "##".into()] },
            ],
            ..Default::default()
        };

        assert_eq!(24, sut.answer()?);
        Ok(())
    }

    #[test]
    fn test_different_ids() -> Result<(), String> {
        let sut: Stage1 = Stage1 {
            tiles: vec![
                Tile { id: 2, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 3, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 4, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 5, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
            ],
            ..Default::default()
        };

        assert_eq!(120, sut.answer()?);
        Ok(())
    }

    #[test]
    fn test_include_non_corner_tiles() -> Result<(), String> {
        let sut: Stage1 = Stage1 {
            // layout:
            // 1 <1>  2 <2>  3
            // ^      ^      ^
            // 3      4      5
            // v      v      v
            // 4 <6>  5 <7>  6
            // ^      ^      ^
            // 8      9      10
            // v      v      v
            // 7 <11> 8 <12> 9

            // cardinal direction edges like "n.." will have no possible match
            tiles: vec![
                Tile { id: 1, edges: ["n...".into(), "...#".into(), "..##".into(), "w...".into()] },
                Tile { id: 2, edges: ["n...".into(), "..#.".into(), ".#..".into(), "...#".into()] },
                Tile { id: 3, edges: ["n...".into(), "e...".into(), ".#.#".into(), "..#.".into()] },
                Tile { id: 4, edges: ["..##".into(), ".##.".into(), "#...".into(), "w...".into()] },
                Tile { id: 5, edges: ["..#.".into(), ".###".into(), "#..#".into(), ".##.".into()] },
                Tile { id: 6, edges: [".#.#".into(), "e...".into(), "#.#.".into(), ".###".into()] },
                Tile { id: 7, edges: ["#...".into(), "#.##".into(), "s...".into(), "w...".into()] },
                Tile { id: 8, edges: ["#..#".into(), "##..".into(), "s...".into(), "#.##".into()] },
                Tile { id: 9, edges: ["#.#.".into(), "e...".into(), "s...".into(), "##..".into()] },
            ],
            ..Default::default()
        };

        assert_eq!(1 * 3 * 7 * 9, sut.answer()?);
        Ok(())
    }
}
