use std::collections::VecDeque;

use crate::recursive_parser::Vocabulary;

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
    pub fn create_tokens(source: String, vocab: &Vocabulary) -> Self {
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
