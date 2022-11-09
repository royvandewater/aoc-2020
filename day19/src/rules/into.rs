use std::collections::HashMap;

use super::{Rule, Rules};

impl Into<HashMap<usize, Rule>> for Rules {
    fn into(self) -> HashMap<usize, Rule> {
        self.0
    }
}
