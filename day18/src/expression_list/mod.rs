mod from_str;

use crate::expression::Expression;

pub struct ExpressionList(Vec<Expression>);

impl ExpressionList {
    pub fn iter(&self) -> std::slice::Iter<Expression> {
        self.0.iter()
    }
}
