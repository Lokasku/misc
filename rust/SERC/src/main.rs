mod eval;

use eval::Eval;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut expr = Eval::new(args[1].clone());
    expr.tokenize();
    expr.eval();
    println!("{:?}", expr.stack.get(0).unwrap());
}