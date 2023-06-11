mod parser;
mod vocab;

//Pub use for outer modules.
pub use vocab::Vocabulary;

//use for inner modules.
use parser::{Token, Tokenizer};

pub fn parse_input(source: String, vocab: &Vocabulary) {
    let mut tokens = Tokenizer::create_tokens(source, vocab);
    sentence_expr(&mut tokens);
    match tokens.get_token() {
        Token::Eof => println!("Parsed Successfully!"),
        _ => (),
    }
}

fn sentence_expr(tokens: &mut Tokenizer) {
    let token = tokens.get_token();
    match token {
        Token::Verb(_) => {
            tokens.next_token();
            object_expr(tokens);
        }
        _ => return,
    }
}
fn object_expr(tokens: &mut Tokenizer) {
    let mut token = tokens.get_token();
    match token {
        Token::Adjective(_) => tokens.next_token(),
        _ => (),
    };
    token = tokens.get_token();
    match token {
        Token::Noun(_) => tokens.next_token(),
        _ => {
            println!("Object Grammer incorrect");
            return;
        }
    };
}
