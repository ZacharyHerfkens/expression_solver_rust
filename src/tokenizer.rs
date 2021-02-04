pub enum Token {
    Number(f64),
    AdditionOperator,
    SubtractionOperator,
    MultiplicationOperator,
    DivisionOperator,
    OpenParenthesis,
    CloseParenthesis
}

pub struct Tokenizer {
    expr: String,
    index: usize
}

impl Tokenizer {
    pub fn new(expr: String) -> Tokenizer {
        Tokenizer {
            expr: expr,
            index: 0
        }
    }
}