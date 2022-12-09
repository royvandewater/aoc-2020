use std::{collections::HashSet, str::FromStr};

use crate::input::Item;

#[derive(Clone, Debug, Eq, Hash, PartialOrd, Ord, PartialEq)]
pub struct Tile {
    pub id: usize,
    pub edges: [String; 4],
    pub lines: Vec<String>,
    pub flipped: bool,
    pub rotations: usize,
}

impl Tile {
    pub fn possible_edges(&self) -> HashSet<String> {
        self.edges.iter().map(canonize_edge).collect()
    }

    pub fn rotate_matched(&self, shared_edges: &HashSet<String>, edge_mask: [bool; 4]) -> Result<Tile, String> {
        let mut tile = self.clone();

        for _ in 0..4 {
            tile.rotate_once();

            if edge_mask_matches(&shared_edges, edge_mask, &tile.edges) {
                return Ok(tile);
            }
        }

        Err("Failed to find a rotation that matched the mask".into())
    }

    pub fn rotate_once(&mut self) {
        self.lines = rotate_lines(&self.lines);
        self.edges = get_edges(&self.lines);
        self.rotations += 1;
    }

    pub fn flip_vertically(&mut self) {
        self.lines = self.lines.iter().rev().cloned().collect();
    }

    pub fn flip_horizontally(&mut self) {
        self.lines = self.lines.iter().map(|l| l.chars().rev().collect()).collect();
    }
}

pub fn rotate_lines(lines: &Vec<String>) -> Vec<String> {
    let transposed = transpose(&lines);

    let rotated = transposed.iter().map(|line| line.chars().rev().collect::<String>());

    return rotated.collect();
}

fn transpose(lines: &Vec<String>) -> Vec<String> {
    let mut transposed = lines.clone();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            replace_char(&mut transposed, y, x, c)
        }
    }

    return transposed;
}

fn replace_char(lines: &mut Vec<String>, x: usize, y: usize, c: char) {
    if y >= lines.len() {
        panic!("y was too large: {} > {}", y, lines.len() - 1)
    }
    lines[y].replace_range(x..x + 1, c.to_string().as_str());
}

fn edge_mask_matches(shared_edges: &HashSet<String>, edge_mask: [bool; 4], edges: &[String; 4]) -> bool {
    edge_mask.iter().enumerate().all(|(i, &mask)| {
        let edge = canonize_edge(&edges[i]);
        let is_shared_edge = shared_edges.contains(&edge);

        match mask {
            true => is_shared_edge,
            false => !is_shared_edge,
        }
    })
}

pub fn canonize_edge(str: &String) -> String {
    let forward = str.to_string();
    let reversed = str.chars().rev().collect::<String>();

    match reversed < forward {
        true => reversed,
        false => forward,
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self { id: 0, edges: ["".into(), "".into(), "".into(), "".into()], lines: Vec::new(), flipped: false, rotations: 0 }
    }
}

impl From<&Item> for Tile {
    fn from(item: &Item) -> Self {
        Tile { id: item.id, edges: get_edges(&item.tile), lines: item.tile.clone(), ..Default::default() }
    }
}

fn get_edges(lines: &Vec<String>) -> [String; 4] {
    let north = lines.first().unwrap().to_string();
    let south = lines.last().unwrap().chars().rev().collect::<String>();

    let west = lines.iter().map(|l| l.chars().next().unwrap()).rev().collect();
    let east = lines.iter().map(|l| l.chars().last().unwrap()).collect();

    [north, east, south, west]
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let item: Item = s.parse()?;
        Ok(Tile::from(&item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let sut = Tile { edges: ["#".into(), "#".into(), "#".into(), "#".into()], ..Default::default() };
        let expected = HashSet::from(["#".into()]);

        assert_eq!(expected, sut.possible_edges());
    }

    #[test]
    fn test_flipping_doesnt_matter() {
        let sut = Tile { edges: ["#.".into(), "#.".into(), "#.".into(), "#.".into()], ..Default::default() };
        let expected = HashSet::from(["#.".into()]);

        assert_eq!(expected, sut.possible_edges());
    }

    #[test]
    fn test_rotate_once() -> Result<(), String> {
        let item: Item = "
            Tile 56:
            12
            34
        "
        .parse()?;

        let mut sut = Tile::from(&item);
        sut.rotate_once();

        // 31
        // 42
        assert_eq!(vec!["31", "42",], sut.lines);
        Ok(())
    }
}
