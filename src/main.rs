mod parser;
mod vocab;

use parser::{Tokenizer, parse};
use vocab::Vocabulary;

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("go rusty sword");

    let parseRes = parse(input, &vocabulary);
}
