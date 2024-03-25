mod charstream;
mod errors;
mod parser;
mod tokens;

//use crate::tokens::Tokenizer;
use crate::parser::expression;
use crate::tokens::TokenStream;
use std::{
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
        let input = read_user_input().trim().to_string();

        // Temporary graceful exit
        if input == "q".to_string() {
            break;
        }

        let mut ts = match TokenStream::new(input) {
            Some(v) => v,
            None => continue,
        };

        match expression(&mut ts) {
            Ok(ans) => print_result(ans),
            Err(e) => {
                println!("{e}")
            }
        }

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
