mod from_str;

#[derive(Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
}

use std::slice::Iter;

use Operator::Add;
use Operator::Multiply;

pub enum Token {
    Value(isize),
    Operator(Operator),
    OpenParenthesis,
    CloseParenthesis,
}

pub struct Expression(Vec<Token>);

impl Expression {
    pub fn evaluate(&self) -> isize {
        let mut tokens = self.0.iter();
        return evaluate_tokens(&mut tokens);
    }
}

fn evaluate_tokens(tokens: &mut Iter<Token>) -> isize {
    let mut accumulator = 0;
    let mut operator: Operator = Add;

    while let Some(token) = tokens.next() {
        match token {
            Token::Value(v) => match operator {
                Add => accumulator += v,
                Multiply => accumulator *= v,
            },
            Token::Operator(o) => operator = *o,
            Token::OpenParenthesis => match operator {
                Add => accumulator += evaluate_tokens(tokens),
                Multiply => accumulator *= evaluate_tokens(tokens),
            },
            Token::CloseParenthesis => return accumulator,
        }
    }

    return accumulator;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_1_plus_1() -> Result<(), String> {
        let sut: Expression = "1 + 1".parse()?;

        assert_eq!(2, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_1_times_1() -> Result<(), String> {
        let sut: Expression = "1 * 1".parse()?;

        assert_eq!(1, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_1_plus_2_times_3() -> Result<(), String> {
        let sut: Expression = "1 + 2 * 3".parse()?;

        assert_eq!(9, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_parenthesis() -> Result<(), String> {
        let sut: Expression = "1 + (2 * 3)".parse()?;

        assert_eq!(7, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_1() -> Result<(), String> {
        let sut: Expression = "2 * 3 + (4 * 5)".parse()?;

        assert_eq!(26, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<(), String> {
        let sut: Expression = "5 + (8 * 3 + 9 + 3 * 4 * 3)".parse()?;

        assert_eq!(437, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<(), String> {
        let sut: Expression = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".parse()?;

        assert_eq!(12240, sut.evaluate());
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<(), String> {
        let sut: Expression = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".parse()?;

        assert_eq!(13632, sut.evaluate());
        Ok(())
    }
}
