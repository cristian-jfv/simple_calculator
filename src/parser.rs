// Grammar proposed by B. Stroustrup in Programming Principles and Practice Using C++
// Expression:
//  Term
//  Expression "+" Term : addition
//  Expression "-" Term : substraction
// Term:
//  Primary
//  Term "*" Primary : multiplication
//  Term "/" Primary : division
//  Term "%" Primary : remainder (modulo)
// Primary:
//  Number
//  "(" Expression ")" : grouping
// Number:
//  floating-point-literal

use crate::tokens::{TokenKind, Tokenizer};

const VERBOSE: bool = false;

fn log(message: &str) {
    if !VERBOSE {
        return;
    }
    println!("{}", message.to_string());
}

// Deal with + and -
pub fn expression(tzr: &mut Tokenizer) -> f64 {
    log("entering expression");
    let mut left = term(tzr); // read and evaluate a term
    let mut t = tzr.get_token(); // get the next token

    loop {
        if t.is_none() {
            break;
        }
        match t.clone().unwrap().kind {
            TokenKind::Plus => {
                log("expression finds a sum");
                left += term(tzr);
                t = tzr.get_token();
            }
            TokenKind::Minus => {
                log("expression finds a substraction");
                left -= term(tzr);
                t = tzr.get_token();
            }
            _ => {
                log("expression finds no match");
                tzr.put_back(t.unwrap());
                break;
            }
        }
    }
    log("expression returns {left}");
    return left;
}

// Deal with *, /, and %
pub fn term(tzr: &mut Tokenizer) -> f64 {
    log("entering term");
    let mut left = primary(tzr);
    let mut t = tzr.get_token();
    loop {
        if t.is_none() {
            break;
        }

        match t.clone().unwrap().kind {
            TokenKind::Times => {
                log("term finds a multiplication");
                left *= primary(tzr);
                t = tzr.get_token();
            }
            TokenKind::Divide => {
                log("term finds a division");
                let d = primary(tzr);
                if d == 0.0 {
                    eprintln!("Found division by zero");
                    break; //process::exit(1);
                }
                left /= d;
                t = tzr.get_token();
            }
            _ => {
                log("no match at term");
                tzr.put_back(t.clone().unwrap());
                break;
            }
        }
    }
    log("term returns {left}");
    return left;
}

// Deal with numbers and parentheses
pub fn primary(tzr: &mut Tokenizer) -> f64 {
    log("entering primary");
    let t = tzr.get_token();
    if t.is_none() {
        return 0.0;
    }
    let t = t.unwrap();

    match t.kind {
        TokenKind::OpenParenthesis => {
            log("primary finds an opening parenthesis");
            let d = expression(tzr);
            let t = tzr.get_token();

            match t.unwrap().kind {
                TokenKind::CloseParenthesis => {
                    log("primary finds a closing parenthesis");
                    return d;
                },
                _ => {
                    eprintln!("primary expected ')'");
                    return 0.0;
                }
            }
        }
        TokenKind::Number => {
            log("primary finds a number");
            //println!("primary returns {}", t.value);
            return t.value;
        }
        _ => {
            eprintln!("primary expected");
            return 0.0;
        }
    }
}
