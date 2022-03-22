#![feature(associated_type_bounds)]
use std::cell::RefCell;
mod dimension;
mod equation;
mod quantity;
mod unit;
use equation::*;

fn main() {
    let rl = RefCell::new(rustyline::Editor::<()>::new());
    let input: Box<dyn Iterator<Item = _>> = if std::env::args().len() > 1 {
        Box::new((0..1).map(|_| std::env::args().skip(1).collect::<Vec<String>>().join(" ")))
    } else {
        Box::new(
            (0..)
                .map(|_| rl.borrow_mut().readline(">>> "))
                .take_while(|i| i.is_ok())
                .map(|i| i.unwrap()),
        )
    };
    let parser = parser();
    let evaler = semanter();
    for expr in input {
        let tokens = tokenizer(expr.chars());
        match parser.parse(tokens) {
            Err(e) => println!("Parse err: {:?}", e),
            Ok(state) => {
                rl.borrow_mut().add_history_entry(&expr);
                let val = evaler.eval(&state);
                if let Ok(v) = val {
                    println!("{}", v);
                } else {
                    println!("{:?}", val);
                }
            }
        }
    }
}
