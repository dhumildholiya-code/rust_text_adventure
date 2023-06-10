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
    pub fn get_token(&self) -> Token {
        let first = self.tokens.front();
        first.expect("There are no tokens!").to_owned()
    }
    pub fn next_token(&mut self) {
        self.tokens.pop_front().expect("There are no tokens");
    }
}

pub fn parse(source: String, vocab: &Vocabulary) -> Result<(), String> {
    let mut tokens = get_tokens(source, vocab)?;
    // dbg!(tokens);
    sentence(&mut tokens);
    if tokens.get_token().token_type == TokenType::Eof {
        println!("Parsed Successfully!");
    }
    Ok(())
}

fn sentence(tokens: &mut Tokenizer) {
    let token = tokens.get_token();
    if token.token_type == TokenType::Verb {
        tokens.next_token();
        object(tokens);
    }
}
fn object(tokens: &mut Tokenizer) {
    let mut token = tokens.get_token();
    if token.token_type == TokenType::Noun {
        tokens.next_token();
    } else if token.token_type == TokenType::Adjective {
        tokens.next_token();
        token = tokens.get_token();
        if token.token_type == TokenType::Noun {
            tokens.next_token();
        } else {
            println!("Error parsing");
        }
    } else {
        println!("Grammer Not Correct");
        return;
    }
}

fn get_tokens(source: String, vocab: &Vocabulary) -> Result<Tokenizer, String> {
    let mut tokens = Tokenizer::new();
    let mut source = source.as_str().split_whitespace();
    while let Some(word) = source.next() {
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
