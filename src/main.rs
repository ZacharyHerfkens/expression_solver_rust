mod tokenizer;
use tokenizer::{Token, Tokenizer};

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please provide an expression to evaluate");
        return;
    }

    let expr = args.pop().unwrap();

    //tokenize the expression
    let token_stream = Tokenizer::new(&expr);

    for t in token_stream {
        println!("{:?}", t);
    }
}