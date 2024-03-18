mod tokens;

use crate::tokens::Tokenizer;
use std::io::{self, Write};

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

fn main() {
    loop {
        print_prompt();
        let input = read_user_input();

        // Temporary graceful exit
        if input.trim() == "q".to_string() {
            break;
        }

        let mut tzr = Tokenizer::new(input);

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
