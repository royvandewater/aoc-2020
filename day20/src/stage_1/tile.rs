use std::collections::HashSet;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Tile {
    pub id: usize,
    pub edges: [String; 4],
}

impl Tile {
    pub fn possible_edges(&self) -> HashSet<String> {
        let mut result: HashSet<String> = self.edges.iter().cloned().collect();

        result.extend(self.edges.iter().map(reverse_string));

        return result;
    }
}

fn reverse_string(str: &String) -> String {
    str.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let sut = Tile { id: 1, edges: ["#".into(), "#".into(), "#".into(), "#".into()] };
        let expected = HashSet::from(["#".into()]);

        assert_eq!(expected, sut.possible_edges());
    }

    #[test]
    fn test_flipping_matters() {
        let sut = Tile { id: 1, edges: ["#.".into(), "#.".into(), "#.".into(), "#.".into()] };
        let expected = HashSet::from(["#.".into(), ".#".into()]);

        assert_eq!(expected, sut.possible_edges());
    }
}
