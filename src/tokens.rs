use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Plus,
    Minus,
    Times,
    Divide,
    OpenParenthesis,
    CloseParenthesis,
    Number,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Times => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::OpenParenthesis => write!(f, "("),
            TokenKind::CloseParenthesis => write!(f, ")"),
            TokenKind::Number => write!(f, "5"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: f64,
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: VecDeque<Token>,
    verbose: bool,
}

impl Tokenizer {
    // Constructor
    pub fn new(input: String, verbose: bool) -> Tokenizer {
        Tokenizer {
            tokens: Self::parse_tokens(input),
            verbose: verbose,
        }
    }

    fn print_token(t: &Token) {
        match t.kind {
            TokenKind::Number => print!(" {} ", t.value),
            _ => print!(" {} ", t.kind),
        }
    }

    fn print_tokens(&self) {
        if self.verbose && self.tokens.is_empty() {
            println!(" token list is empty");
            return;
        }
        for t in &self.tokens {
            Self::print_token(t);
        }
        println!("");
    }

    pub fn get_token(&mut self) -> Option<Token> {
        let t = self.tokens.pop_front();
        /*if t.is_none() { 
            println!("token list is empty");
            return t;
        }
        print!("popping [");
        Self::print_token(&t.clone().unwrap());
        print!("]");
        Self::print_tokens(self);*/
        return t;
    }

    pub fn put_back(&mut self, token: Token) {
        /*print!("returning [");
        Self::print_token(&token);
        print!("]");
        Self::print_tokens(self);*/
        self.tokens.push_front(token);
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
                Some(' ') => {}
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
                        }
                        None => {
                            eprintln!("error while parsing number");
                            break;
                        }
                    }
                }
                _ => {}
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
            } else if chr == '-' && input.chars().nth(index - 1) == Some('e') {
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
