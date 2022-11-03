use std::str::FromStr;

use super::{Add, Expression, Formula, Multiply};

impl FromStr for Formula {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Ok(Formula(Vec::new()));
        }

        match s.find("(") {
            None => {
                let expressions = s
                    .trim()
                    .split(' ')
                    .map(|token_str| token_str.parse())
                    .collect::<Result<Vec<Expression>, String>>()?;

                return Ok(Formula(expressions));
            }
            Some(i) => {
                let j = find_closing_parenthesis(s, i)
                    .ok_or(format!("Could not find closing parenthesis for: {}", s))?;

                let before: Formula = s[..i].parse()?;
                let middle: Formula = s[i + 1..j].parse()?;
                let after: Formula = s[j + 1..].parse()?;

                let mut expressions: Vec<Expression> = Vec::new();
                expressions.extend(before.0);
                expressions.push(Expression::Formula(middle));
                expressions.extend(after.0);
                return Ok(Formula(expressions));
            }
        }
    }
}

fn find_closing_parenthesis(s: &str, open_index: usize) -> Option<usize> {
    let mut open_count = 0;

    for (i, char) in s.chars().enumerate().skip(open_index) {
        match char {
            '(' => open_count += 1,
            ')' => open_count -= 1,
            _ => (),
        }

        if open_count == 0 {
            return Some(i);
        }
    }

    return None;
}

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Expression::Operator(Add)),
            "*" => Ok(Expression::Operator(Multiply)),
            s => Ok(Expression::Value(isize::from_str(s).map_err(|e| {
                format!("failed to parse non-operator as int: {}, {}", e, s)
            })?)),
        }
    }
}
