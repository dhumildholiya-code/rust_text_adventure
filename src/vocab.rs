use std::collections::HashMap;

use crate::parser::TokenType;

pub struct Vocabulary{
    vocabs : Vec<HashMap<String, u32>>,
}
impl Vocabulary {
    pub fn check_word(&self, word:&str)->TokenType{
        let mut i: u32 = 0;
        for vocab in self.vocabs.iter(){
            if vocab.contains_key(word){
                break;
            }
            i += 1;
        }
        let word_type = match i {
            0 => TokenType::Verb, 
            1 => TokenType::Noun,
            2 => TokenType::Adjective,
            _ => TokenType::Nil,
        };
        word_type
    }
    pub fn new()->Self{
       let verbs = HashMap::from([
                                 ("go".to_string(), 0),
                                 ("pick".to_string(), 1),
       ]);
       let nouns = HashMap::from([
                                 ("north".to_string(), 0),
                                 ("west".to_string(), 1),
                                 ("east".to_string(), 2),
                                 ("sword".to_string(), 3),
                                 ("stone".to_string(), 4),
       ]);
       let adjectives = HashMap::from([
                                 ("rusty".to_string(), 0),
                                 ("iron".to_string(), 1),
                                 ("round".to_string(), 2),
       ]);
       Vocabulary { vocabs: vec![
           verbs,
           nouns,
           adjectives
       ] }
   } 
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn return_verb_type_if_word_is_verb() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("go".to_string());
        assert_eq!(result, TokenType::Verb);
    }
    #[test]
    fn return_noun_type_if_word_is_noun() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("sword".to_string());
        assert_eq!(result, TokenType::Noun);
    }
    #[test]
    fn return_adjective_type_if_word_is_adjecetive() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("rusty".to_string());
        assert_eq!(result, TokenType::Adjective);
    }
    #[test]
    fn return_nil_type_if_word_is_nil() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("garbage".to_string());
        assert_eq!(result, TokenType::Nil);
    }
}
