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

use crate::tokens::Token;
use crate::tokens::Kind;

// Read expression and compose tokens
pub fn get_token(arg: Type) -> Token {
    let mut ch: char;
    
}

// Deal with + and -
pub fn expression(arg: Type) -> f64 {
    let mut left = term();      // read and evaluate a term
    let mut t = get_token();    // get the next token

    loop {
        match t.kind {
            Kind::Plus => {
                left += term();
                t = get_token();
            },
            Kind::Minus => {
                left -= term();
                t = get_token();
            },
            None => break,
        }
    }
    return left;
}

// Deal with *, /, and %
pub fn term(arg: Type) -> f64 {
    let mut left = primary();
    let mut t = get_token();
    loop {
        match t.kind {
            Kind::Times => {
                left *= primary();
                t = get_token();
            },
            Kind::Divide => {
                let d = primary();
                if d == 0 {
                    eprintln!("Found division by zero");
                    process::exit(1);
                }
                left /= d;
                t = get_token();
            },
            None => break
        }
    }
    return left;
}

// Deal with numbers and parentheses
pub fn primary(arg: Type) -> f64 {
    let mut t = get_token();
    match t.kind {
        Kind::OpenParenthesis => {
            let d = expression();
            t = get_token();
            if t.kind != Kind::CloseParenthesis {
                eprintln!("')' expected");
                //process::exit(1);
                return d;
            }
        },
        Kind::Number => return t.value;
        None => eprintln!("primary expected"),
    }
}
