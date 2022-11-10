#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Tile {
    pub id: usize,
    pub edges: [String; 4],
}
