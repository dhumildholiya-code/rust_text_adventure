use std::collections::HashMap;

use crate::parser::Token;

pub struct Vocabulary{
    vocabs : Vec<HashMap<String, u32>>,
}
impl Vocabulary {
    pub fn check_word(&self, word:&str)->Option<Token>{
        let mut i: u32 = 0;
        for vocab in self.vocabs.iter(){
            if vocab.contains_key(word){
                break;
            }
            i += 1;
        }
         match i {
            0 => Some(Token::Verb(word.to_string())), 
            1 => Some(Token::Noun(word.to_string())),
            2 => Some(Token::Adjective(word.to_string())),
            _ => None,
        }
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
        let result = vocab.check_word("go");
        assert_eq!(result, Some(Token::Verb("go".to_string())));
    }
    #[test]
    fn return_noun_type_if_word_is_noun() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("sword");
        assert_eq!(result, Some(Token::Noun("sword".to_string())));
    }
    #[test]
    fn return_adjective_type_if_word_is_adjecetive() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("rusty");
        assert_eq!(result, Some(Token::Adjective("rusty".to_string())));
    }
    #[test]
    fn return_nil_type_if_word_is_nil() {
        let vocab = Vocabulary::new();
        let result = vocab.check_word("garbage");
        assert_eq!(result.is_none(), true);
    }
}
