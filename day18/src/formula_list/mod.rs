mod from_str;

use crate::formula::Formula;

pub struct FormulaList(Vec<Formula>);

impl FormulaList {
    pub fn iter(&self) -> std::slice::Iter<Formula> {
        self.0.iter()
    }
}
