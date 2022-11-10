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
        if self.layout.borrow().iter().any(have_no_options) {
            return Err("Couldn't solve the layout".into());
        }
        if self.layout.borrow().iter().all(have_one_option) {
            return Ok(());
        }

        let reduced_layout = reduce_layout(self.layout.borrow())?;
        if total_entropy(&reduced_layout) >= total_entropy(&self.layout.borrow()) {
            return Err("Failed to reduce entropy".into());
        }

        self.layout.replace(reduced_layout);
        self.solve()
    }
}

fn reduce_layout(
    layout: std::cell::Ref<HashMap<[isize; 2], HashSet<Tile>>>,
) -> Result<HashMap<[isize; 2], HashSet<Tile>>, String> {
    let mut new_layout = HashMap::new();

    for (position, tiles) in layout.iter() {
        new_layout.insert(
            position.clone(),
            tiles
                .iter()
                .filter(|tile| is_valid_tile(&layout, position, tile))
                .cloned()
                .collect(),
        );
    }

    return Ok(new_layout);
}

fn is_valid_tile(
    layout: &HashMap<[isize; 2], HashSet<Tile>>,
    position: &[isize; 2],
    tile: &Tile,
) -> bool {
    todo!()
}

fn have_one_option<T>((_, options): (T, &HashSet<Tile>)) -> bool {
    options.len() == 1
}

fn have_no_options<T>((_, options): (T, &HashSet<Tile>)) -> bool {
    options.len() == 0
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
            tiles: vec![
                Tile { id: 1, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 2, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 3, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
                Tile { id: 4, edges: ["..".into(), ".#".into(), "#.".into(), "##".into()] },
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
