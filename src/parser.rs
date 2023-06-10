use std::collections::VecDeque;

use crate::vocab::Vocabulary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Verb(String),
    Noun(String),
    Adjective(String),
    Eof,
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: VecDeque<Token>,
}
impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer {
            tokens: VecDeque::new(),
        }
    }
    pub fn add_token(&mut self, token: Token) {
        self.tokens.push_back(token);
    }
    pub fn get_token(&self) -> Token {
        let first = self.tokens.front();
        first.expect("There are no tokens!").to_owned()
    }
    pub fn next_token(&mut self) {
        self.tokens.pop_front().expect("There are no tokens");
    }
}

pub fn parse(source: String, vocab: &Vocabulary) {
    let mut tokens = get_tokens(source, vocab);
    // dbg!(tokens);
    sentence(&mut tokens);
    match tokens.get_token() {
        Token::Eof => println!("Parsed Successfully!"),
        _ => (),
    }
}

fn sentence(tokens: &mut Tokenizer) {
    let token = tokens.get_token();
    match token {
        Token::Verb(_) => {
            tokens.next_token();
            object(tokens);
        }
        _ => return,
    }
}
fn object(tokens: &mut Tokenizer) {
    let mut token = tokens.get_token();
    match token {
        Token::Adjective(_) => tokens.next_token(),
        _ => (),
    };
    token = tokens.get_token();
    match token {
        Token::Noun(_) => tokens.next_token(),
        _ => {
            println!("Object Grammer incorrect");
            return;
        },
    };
}

fn get_tokens(source: String, vocab: &Vocabulary) -> Tokenizer {
    let mut tokens = Tokenizer::new();
    let mut source = source.as_str().split_whitespace();
    while let Some(word) = source.next() {
        match vocab.check_word(word) {
            None => continue,
            Some(token) => tokens.add_token(token),
        };
    }
    tokens.add_token(Token::Eof);
    println!("{:?}", tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_tokens() {}
}
