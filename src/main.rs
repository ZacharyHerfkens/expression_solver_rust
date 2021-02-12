mod solver;
mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();

    //An expression should be provided as a command line argument, check if it's there first
    if args.len() < 2 {
        println!("Please provide an expression to evaluate");
        return;
    }
    //Grab the last (hopefully second) argument, which should be a string representing the expression to solve
    let expr = args.pop().unwrap();

    //To solve the expression, it must first be parsed into tokens which can be fed into a
    //recursive expression solver which will also identify any errors that exist within the
    //passed expression
    let mut token_stream = Tokenizer::new(&expr);

    let _answer = solver::solve(&mut token_stream);
}
