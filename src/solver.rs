
use crate::tokenizer::{Tokenizer, Token};

#[derive(Eq, PartialOrd, PartialEq)]
enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

pub fn solve<'a, T: Iterator<Item = Token<'a>>>(tokens: T) -> Result<f64, String> {
    Ok(0.0)
}

fn match_operator(tokens: &mut Tokenizer) -> Result<Operator, String> {
    if let Some(t) = tokens.next() {
        match t {
            Token::AdditionOperator(..) => Ok(Operator::Add),
            Token::SubtractionOperator(..) => Ok(Operator::Sub),
            Token::MultiplicationOperator(..) => Ok(Operator::Mul),
            Token::DivisionOperator(..) => Ok(Operator::Div),
            Token::OpenParenthesis(i, s) |
            Token::CloseParenthesis(i, s) |
            Token::Number(i, s, ..) |
            Token::InvalidToken(i, s) => Err(format!("At {}, Expected operator, found {}", i, s))
        }
    } else {
        Err(String::from("Expected operator, reached end of expression"))
    }
}

fn match_number(tokens: &mut Tokenizer) -> Result<f64, String> {
    if let Some(t) = tokens.next() {
        use Token::*;
        match t {
            Number(.., n) => Ok(n),
            AdditionOperator(i, s) |
            SubtractionOperator(i, s) |
            MultiplicationOperator(i, s) |
            DivisionOperator(i, s) |
            OpenParenthesis(i, s) |
            CloseParenthesis(i, s) |
            InvalidToken(i, s) => Err(format!("At {}, expected number, found {}", i, s))
            
        }
    } else {
        Err(String::from("Expected number, reached end of expression"))
    }
}