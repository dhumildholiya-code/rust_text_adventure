mod parser;
mod vocab;

use parser::parse;
use vocab::Vocabulary;

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("go moon rusty sword");

    let parseRes = parse(input, &vocabulary);
}
