mod parser;
mod vocab;

use parser::{Tokenizer, get_tokens};
use vocab::Vocabulary;

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("pick rusty sword");

    match get_tokens(input, &vocabulary){
        Ok(tokens) => println!("{:?}", tokens),
        Err(err) => println!("{}",err),
    }
}
