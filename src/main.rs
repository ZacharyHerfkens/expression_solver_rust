mod tokenizer;
mod solver;
use tokenizer::Tokenizer;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please provide an expression to evaluate");
        return;
    }

    let expr = args.pop().unwrap();

    //tokenize the expression
    let mut token_stream = Tokenizer::new(&expr);

    let answer = solver::solve(&mut token_stream);
}