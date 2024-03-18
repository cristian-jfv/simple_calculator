mod tokens;

use crate::tokens::Token;
use crate::tokens::TokenKind;
use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Debug)]
struct Tokenizer {
    tokens: VecDeque<Token>,
}

impl Tokenizer {
    // Constructor
    fn new(input: String) -> Tokenizer {
        Tokenizer {
            tokens: Self::parse_tokens(input),
        }
    }
    fn get_token(&mut self) -> Option<Token> {
        return self.tokens.pop_front();
    }

    fn parse_tokens(input: String) -> VecDeque<Token> {
        let mut token_vec: VecDeque<Token> = VecDeque::new();
        //let end = input.chars().count();
        let mut i = 0;


        // Isolate tokens, do not try to apply the grammar
        loop {
            let c = input.chars().nth(i);
            match c {
                None => break,
                Some(' ') => {},
                Some('+') => token_vec.push_back(Token {
                    kind: TokenKind::Plus,
                    value: 0.0,
                }),
                Some('-') => token_vec.push_back(Token {
                    kind: TokenKind::Minus,
                    value: 0.0,
                }),
                Some('*') => token_vec.push_back(Token {
                    kind: TokenKind::Times,
                    value: 0.0,
                }),
                Some('/') => token_vec.push_back(Token {
                    kind: TokenKind::Divide,
                    value: 0.0,
                }),
                Some('(') => token_vec.push_back(Token {
                    kind: TokenKind::OpenParenthesis,
                    value: 0.0,
                }),
                Some(')') => token_vec.push_back(Token {
                    kind: TokenKind::CloseParenthesis,
                    value: 0.0,
                }),
                Some(d) if d.is_numeric() => {
                    // Handle numerical tokens
                    let (new_index, number) = Self::handle_numeric_tokens(&input, i);
                    match number {
                        Some(d) => {
                            i = new_index;
                            token_vec.push_back(Token {
                                kind: TokenKind::Number,
                                value: d,
                            });
                        },
                        None => {
                            eprintln!("error while parsing number");
                            break;
                        },
                    }
                },
                _ => {},
            }

            i += 1;
        }

        return token_vec;
    }

    fn handle_numeric_tokens(input: &String, mut index: usize) -> (usize, Option<f64>) {
        let mut number_str = String::new();
        // Read
        while let Some(chr) = input.chars().nth(index) {
            if chr.is_numeric() || chr == '.' || chr == 'e' {
                number_str.push(chr);
                index += 1;
            } else if chr == '-' && input.chars().nth(index-1) == Some('e') {
                // Check previous character, it should be 'e'
                number_str.push(chr);
                index += 1;
            } else {
                break;
            }
        }
        // Parse
        if let Ok(number) = number_str.parse::<f64>() {
            return (index - 1, Some(number));
        } else {
            return (index - 1, None);
        }
    }
}

fn main() {
    println!("");
    print!(">");
    io::stdout().flush().expect("Buffer flush error");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut tzr = Tokenizer::new(input);

    //while let token = tzr.get_token().unwrap() {
    //    println!("{}", token.kind);
    //}

    loop {
        let token = tzr.get_token();
        if token.is_none() {
            break;
        }
        let token = token.unwrap();
        println!("kind {}, value: {}", token.kind, token.value);
    }

    println!("");
}
