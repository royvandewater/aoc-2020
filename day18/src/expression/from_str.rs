use std::str::FromStr;

use super::{Add, Expression, Multiply, Token};

impl FromStr for Expression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<Token> = s
            .split("(")
            .collect::<Vec<&str>>()
            .join("( ")
            .split(")")
            .collect::<Vec<&str>>()
            .join(" )")
            .split(' ')
            .map(|token_str| token_str.parse())
            .collect::<Result<Vec<Token>, String>>()?;

        Ok(Expression(tokens))
    }
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Token::Operator(Add)),
            "*" => Ok(Token::Operator(Multiply)),
            "(" => Ok(Token::OpenParenthesis),
            ")" => Ok(Token::CloseParenthesis),
            s => Ok(Token::Value(isize::from_str(s).map_err(|e| {
                format!("failed to parse non-operator as int: {}, {}", e, s)
            })?)),
        }
    }
}
