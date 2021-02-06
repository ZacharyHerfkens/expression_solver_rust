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
    chars: std::str::CharIndices<'a>,
    current: Option<(usize, char)>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &str) -> Tokenizer {
        let mut chars = expr.char_indices();
        let current = chars.next();
        Tokenizer {
            expr: expr,
            chars: chars,
            current: current,
        }
    }
    fn advance(&mut self) {
        self.current = self.chars.next();
    }

    fn consume_number(&mut self) -> Option<(&'a str, usize)> {
        let start;
        let mut end = 0;

        if let Some((s, _)) = self.current {
            start = s;
        } else {
            return None;
        }

        while let Some((cur, c)) = self.current {
            if !c.is_numeric() {
                break;
            }
            end = cur;
            self.advance();
        }

        self.expr.get(start..end+1).and_then(|v| Some((v, start)))
    }

    fn consume_float(&mut self) -> Option<(&'a str, usize)> {
        let whole = self.consume_number();
        let dec = self.consume_character('.');
        let frac = self.consume_number();
        None
    }

    fn consume_character(&mut self, k: char) -> Option<(&'a str, usize)> {
        if let Some((s, c)) = self.current {
            if c == k {
                self.advance();
                let end = self.current.map_or_else(|| self.expr.len(), |v| v.0);
                return self.expr.get(s..end).and_then(|v| Some((v, s)));
            }
        }

        None
    }

    fn consume_whitespace(&mut self) {
        if let Some((_, c)) = self.current {
            if c.is_whitespace() {
                self.advance();
            } else {
                return;
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.consume_whitespace();
        if self.current.is_none() {
            return None;
        } else if let Some((s, i)) = self.consume_number() {
            if let Ok(num) = s.parse() {
                return Some(Token::Number(num, i));
            } else {
                return Some(Token::InvalidToken(i));
            }
        } else if let Some((_, i)) = self.consume_character('+') {
            return Some(Token::AdditionOperator(i));
        } else if let Some((_, i)) = self.consume_character('-') {
            return Some(Token::SubtractionOperator(i));
        } else if let Some((_, i)) = self.consume_character('*') {
            return Some(Token::MultiplicationOperator(i));
        } else if let Some((_, i)) = self.consume_character('/') {
            return Some(Token::DivisionOperator(i));
        } else if let Some((_, i)) = self.consume_character('(') {
            return Some(Token::OpenParenthesis(i));
        } else if let Some((_, i)) = self.consume_character(')') {
            return Some(Token::CloseParenthesis(i));
        } else if let Some((i, _)) = self.current {
            return Some(Token::InvalidToken(i));
        } else {
            return None;
        }
    }
}
