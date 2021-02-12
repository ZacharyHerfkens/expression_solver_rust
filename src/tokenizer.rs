use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref num_regex: Regex = Regex::new(r"^([0-9]+(\.[0-9]*)?)|([0-9]*\.[0-9]+)").unwrap();
    static ref whitespace_regex: Regex = Regex::new(r"^[\s]+").unwrap();
    static ref operator_regex: Regex = Regex::new(r"^[+\-\*/\(\)]").unwrap();
}

#[derive(Debug)]
pub enum Token<'a> {
    Number(usize, &'a str, f64),
    AdditionOperator(usize, &'a str),
    SubtractionOperator(usize, &'a str),
    MultiplicationOperator(usize, &'a str),
    DivisionOperator(usize, &'a str),
    OpenParenthesis(usize, &'a str),
    CloseParenthesis(usize, &'a str),
    InvalidToken(usize, &'a str),
}

pub struct Tokenizer<'a> {
    expr: &'a str,
    index: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer {
        Tokenizer { expr, index: 0 }
    }
    fn consume(&mut self, m: &Regex) -> Option<(&'a str, usize)> {
        let start = self.index;
        if let Some(k) = m.find(&self.expr[self.index..]) {
            let s = k.as_str();
            self.index = k.end() + start;
            Some((s, start))
        } else {
            None
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.consume(&whitespace_regex);
        if let Some((s, i)) = self.consume(&num_regex) {
            if let Ok(n) = s.parse() {
                return Some(Token::Number(i, s, n));
            } else {
                return Some(Token::InvalidToken(i, s));
            }
        } else if let Some((s, i)) = self.consume(&operator_regex) {
            match s {
                "+" => return Some(Token::AdditionOperator(i, s)),
                "-" => return Some(Token::SubtractionOperator(i, s)),
                "*" => return Some(Token::MultiplicationOperator(i, s)),
                "/" => return Some(Token::DivisionOperator(i, s)),
                "(" => return Some(Token::OpenParenthesis(i, s)),
                ")" => return Some(Token::CloseParenthesis(i, s)),
                _ => return Some(Token::InvalidToken(i, s)),
            }
        }

        None
    }
}
