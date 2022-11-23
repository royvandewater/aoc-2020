mod from;
mod tile;

use std::collections::HashSet;

use tile::Tile;

pub struct Stage1 {
    tiles: Vec<Tile>,
}

impl Stage1 {
    pub fn answer(&self) -> Result<usize, String> {
        let shared_edges = self.shared_edges();

        let corner_tile_ids = self
            .tiles
            .iter()
            .filter(|&t| edge_count_is_2(&shared_edges, t))
            .map(|t| t.id);

        let product = corner_tile_ids.product();

        Ok(product)
    }

    fn shared_edges(&self) -> HashSet<String> {
        let mut edges: HashSet<String> = HashSet::new();
        let mut shared_edges: HashSet<String> = HashSet::new();
        self.tiles
            .iter()
            .flat_map(|t| t.possible_edges())
            .for_each(|edge| {
                // There's a potential bug here where two identical edges on the same tile could potentially be considered shared
                if edges.contains(&edge) {
                    shared_edges.insert(edge.clone());
                }
                edges.insert(edge.clone());
            });
        shared_edges
    }
}

fn edge_count_is_2(shared_edges: &HashSet<String>, tile: &Tile) -> bool {
    let shared_edge_count = tile
        .edges
        .iter()
        .filter(|&e| shared_edges.contains(e))
        .count();

    shared_edge_count == 2
}

impl Default for Stage1 {
    fn default() -> Self {
        Self { tiles: Vec::new() }
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
                Tile { id: 1, edges: ["n1".into(), "..".into(), ".#".into(), "w1".into()] },
                Tile { id: 2, edges: ["n2".into(), "e2".into(), "#.".into(), "..".into()] },
                Tile { id: 3, edges: [".#".into(), "##".into(), "s3".into(), "w3".into()] },
                Tile { id: 4, edges: ["#.".into(), "e4".into(), "s4".into(), "##".into()] },
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
                Tile { id: 2, edges: ["n1".into(), "..".into(), ".#".into(), "w1".into()] },
                Tile { id: 3, edges: ["n2".into(), "e2".into(), "#.".into(), "..".into()] },
                Tile { id: 4, edges: [".#".into(), "##".into(), "s3".into(), "w3".into()] },
                Tile { id: 5, edges: ["#.".into(), "e4".into(), "s4".into(), "##".into()] },
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
                Tile { id: 1, edges: ["n..1".into(), "...#".into(), "..##".into(), "w..1".into()] },
                Tile { id: 2, edges: ["n..2".into(), "..#.".into(), ".#..".into(), "...#".into()] },
                Tile { id: 3, edges: ["n..3".into(), "e..3".into(), ".#.#".into(), "..#.".into()] },
                Tile { id: 4, edges: ["..##".into(), ".##.".into(), "#...".into(), "w..4".into()] },
                Tile { id: 5, edges: ["..#.".into(), ".###".into(), "#..#".into(), ".##.".into()] },
                Tile { id: 6, edges: [".#.#".into(), "e..6".into(), "#.#.".into(), ".###".into()] },
                Tile { id: 7, edges: ["#...".into(), "#.##".into(), "s..7".into(), "w..7".into()] },
                Tile { id: 8, edges: ["#..#".into(), "##..".into(), "s..8".into(), "#.##".into()] },
                Tile { id: 9, edges: ["#.#.".into(), "e.10".into(), "s..9".into(), "##..".into()] },
            ],
            ..Default::default()
        };

        assert_eq!(1 * 3 * 7 * 9, sut.answer()?);
        Ok(())
    }
}
