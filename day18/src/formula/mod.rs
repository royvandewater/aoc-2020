mod from_str;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Add,
    Multiply,
}

use Operator::Add;
use Operator::Multiply;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Value(isize),
    Operator(Operator),
    Formula(Formula),
}

impl Into<isize> for Expression {
    fn into(self) -> isize {
        match self {
            Expression::Value(v) => v,
            _ => panic!("Expression into<isize> called for non value: {:?}", &self),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Formula(Vec<Expression>);

impl Formula {
    pub fn evaluate(&self) -> isize {
        let mut expressions: Vec<Expression> = self.0.clone();

        expressions = evaluate_formulae(expressions);
        expressions = evaluate_additions(expressions);
        return evaluate_multiplications(expressions);
    }
}

fn evaluate_formulae(expressions: Vec<Expression>) -> Vec<Expression> {
    expressions
        .iter()
        .map(|expression| match expression {
            Expression::Formula(f) => Expression::Value(f.evaluate()),
            _ => expression.clone(),
        })
        .collect()
}

fn evaluate_additions(mut expressions: Vec<Expression>) -> Vec<Expression> {
    match expressions
        .iter()
        .position(|e| e == &Expression::Operator(Add))
    {
        None => expressions,
        Some(i) => {
            let begin = i - 1;
            let end = i + 1;

            let first: isize = expressions[begin].clone().into();
            let second: isize = expressions[end].clone().into();
            let sum = Expression::Value(first + second);

            expressions.splice(begin..=end, [sum]);

            return evaluate_additions(expressions);
        }
    }
}

fn evaluate_multiplications(expressions: Vec<Expression>) -> isize {
    let mut accumulator: isize = 1;

    for expression in expressions.iter() {
        match expression {
            Expression::Value(v) => accumulator *= v,
            Expression::Formula(_) => panic!("evaluate_multiplications called while at least one expression was still a Formula: {:?}", expressions),
            Expression::Operator(o) => match o {
                Multiply => (),
                Add => panic!("evaluate_multiplications called while at least one expression was still a + operator: {:?}", expressions),
            },
        }
    }

    return accumulator;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1_plus_1() -> Result<(), String> {
        let sut: Formula = "1 + 1".parse()?;

        assert_eq!(2, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_1_times_1() -> Result<(), String> {
        let sut: Formula = "1 * 1".parse()?;

        assert_eq!(1, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_1_plus_2_times_3() -> Result<(), String> {
        let sut: Formula = "1 + 2 * 3".parse()?;

        assert_eq!(9, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_1_times_2_plus_3() -> Result<(), String> {
        let sut: Formula = "1 * 2 + 3".parse()?;

        assert_eq!(5, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_parenthesis() -> Result<(), String> {
        let sut: Formula = "1 + (2 * 3)".parse()?;

        assert_eq!(7, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_1() -> Result<(), String> {
        let sut: Formula = "2 * 3 + (4 * 5)".parse()?;

        assert_eq!(46, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let sut: Formula = "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse()?;

        assert_eq!(1445, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<(), String> {
        let sut: Formula = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".parse()?;

        assert_eq!(669060, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<(), String> {
        let sut: Formula = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".parse()?;

        assert_eq!(23340, sut.evaluate());
        Ok(())
    }
}
