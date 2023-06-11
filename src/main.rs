mod recursive_parser;

use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("go moon rusty sword");

    let parseRes = parse_input(input, &vocabulary);
}
