mod recursive_parser;

use recursive_parser::{parse, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("go moon rusty sword");

    let parseRes = parse(input, &vocabulary);
}
