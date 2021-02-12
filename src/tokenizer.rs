use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"^([0-9]+(\.[0-9]*)?)|([0-9]*\.[0-9]+)").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"^[\s]+").unwrap();
    static ref OPERATOR_REGEX: Regex = Regex::new(r"^[+\-\*/\(\)]").unwrap();
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
    ///The tokenizer parses this string.
    expr: &'a str,
    ///The starting index used to 'consume' the next token in the string.
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
    ///Create a string showing the character at self.index and a few surrounding characters for the
    ///purpose of deliniating an erroneous character in the string.
    fn error_slice(&self) -> &'a str {
        const NUM_SURROUNDING_CHARS: usize = 5;
        //clamp the start and end indexes to the bounds of the expr string
        let start_index = max(self.index - NUM_SURROUNDING_CHARS, 0);
        let end_index = min(self.index + NUM_SURROUNDING_CHARS + 1, self.expr.len());

        &self.expr[start_index..end_index]
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        //First, see if the current index is within the expression string being parsed.
        if self.index > self.expr.len() {
            //The tokenizer has reached the end of the string and needs to stop tokenizing.
            return None;
        }
        //The goal here is to ignore any whitespace, so consume it if there is any
        self.consume(&WHITESPACE_REGEX);

        //Here, check if the characters next in the string match the regex for detecting numbers, if so, try to parse it.
        //This should always pass because the regex guarantees a valid floating point number is parsed.
        if let Some((s, i)) = self.consume(&NUM_REGEX) {
            if let Ok(n) = s.parse() {
                return Some(Token::Number(i, s, n));
            } else {
                //Not really plausable unless a mistake in the regex exists
                return Some(Token::InvalidToken(i, s));
            }
        //The only other tokens possible are single character operator/parenthesis tokens, consume them if they exist
        } else if let Some((s, i)) = self.consume(&OPERATOR_REGEX) {
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
        //Here, there is a
        Some(Token::InvalidToken(self.index, self.error_slice()))
    }
}
