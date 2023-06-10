use std::{collections::VecDeque, fmt::format};

use crate::vocab::Vocabulary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Verb,
    Noun,
    Adjective,
    Eof,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}
impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }
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
    pub fn get_token(&mut self) -> Token {
        self.tokens
            .pop_front()
            .expect("There are no tokens in Tokenizer")
    }
}

pub fn get_tokens(source: String, vocab: &Vocabulary) -> Result<Tokenizer, String> {
    let mut tokens = Tokenizer::new();
    let mut source = source.as_str().split_whitespace();
    while let Some(word) = source.next() {
        println!("{}", word);
        match vocab.check_word(word) {
            TokenType::Nil => {
                return Err(format!("I don't understand word [{}]", word));
            }
            token_type => {
                let token = Token::new(token_type, word.to_string());
                tokens.add_token(token);
            }
        };
    }
    let token = Token::new(TokenType::Eof, "".to_string());
    tokens.add_token(token);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_tokens() {}
}
