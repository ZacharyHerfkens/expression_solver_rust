#[derive(Debug)]
pub enum Token {
    Number(f64, usize),
    AdditionOperator(usize),
    SubtractionOperator(usize),
    MultiplicationOperator(usize),
    DivisionOperator(usize),
    OpenParenthesis(usize),
    CloseParenthesis(usize),
    InvalidToken(usize),
}

pub struct Tokenizer<'a> {
    expr: &'a str,
    index: usize
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer {
        Tokenizer {
            expr: expr,
            index: 0
        }
    }

}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        
    }
}
