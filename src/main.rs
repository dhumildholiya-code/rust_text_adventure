mod recursive_parser;

use recursive_parser::{parse_input, Vocabulary};

fn main() {
    let vocabulary: Vocabulary = Vocabulary::new();
    let input = String::from("pick rusty sword");

    match parse_input(input, &vocabulary) {
        Ok(_) => (println!("parsed successfully.")),
        Err(err) => println!("{}", err),
    }
}
