use std::collections::HashMap;

mod from_str;

pub type Position = (isize, isize, isize);
#[derive(Clone)]
pub struct State(HashMap<Position, bool>);

impl State {
    pub fn iter(&self) -> std::collections::hash_map::Iter<(isize, isize, isize), bool> {
        self.0.iter()
    }
}

impl From<HashMap<Position, bool>> for State {
    fn from(input: HashMap<Position, bool>) -> Self {
        State(input)
    }
}

impl Into<HashMap<Position, bool>> for &State {
    fn into(self) -> HashMap<Position, bool> {
        self.0.clone()
    }
}
