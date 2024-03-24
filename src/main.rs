mod parser;
mod tokens;

use crate::parser::expression;
use crate::tokens::Tokenizer;
use std::{
    fmt::format,
    io::{self, Write},
    ops::Rem,
};

fn print_prompt() {
    println!("");
    print!("> ");
    io::stdout().flush().expect("Buffer flush error");
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn print_result(result: f64) {
    let order = result.log10();
    let prompt = "= ";
    let output;
    if order.abs() < 4.0 || result.rem(1.0) == 0.0 {
        output = format!("{result}");
    } else {
        output = format!("{result:.5e}");
    }
    println!("{prompt}{output}")
}

fn main() {
    loop {
        print_prompt();
        let input = read_user_input();

        // Temporary graceful exit
        if input.trim() == "q".to_string() {
            break;
        }

        let mut tzr = Tokenizer::new(input, true);

        let result = expression(&mut tzr);
        print_result(result);

        //        loop {
        //            let token = tzr.get_token();
        //            if token.is_none() {
        //                break;
        //            }
        //            let token = token.unwrap();
        //            println!("kind {}, value: {}", token.kind, token.value);
        //        }
    }

    println!("");
}
