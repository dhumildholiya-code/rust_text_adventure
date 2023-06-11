mod parser;
mod vocab;

//Pub use for outer modules.
pub use vocab::Vocabulary;

//use for inner modules.
use parser::{Node, ParseTree, Token, Tokenizer};

pub fn parse_input(source: String, vocab: &Vocabulary) -> Result<ParseTree, String> {
    let mut tokens = Tokenizer::create_tokens(source, vocab);
    let parse_tree = ParseTree::new(sentence_expr(&mut tokens)?);
    println!("{:?}", parse_tree);
    Ok(parse_tree)
}

fn sentence_expr(tokens: &mut Tokenizer) -> Result<Box<Node>, String> {
    let token = tokens.get_token();
    match token {
        Token::Verb(verb) => {
            tokens.next_token();
            return Node::Action {
                action: Node::Value(verb).into(),
                object: object_expr(tokens)?,
            }
            .into();
        }
        _ => {
            return Err(format!(
                "First word is expected to be [Verb/Action].\n <Sentence> -> <Verb> <Object>"
            ))
        }
    }
}
fn object_expr(tokens: &mut Tokenizer) -> Result<Box<Node>, String> {
    let token = tokens.get_token();
    match token {
        Token::Adjective(adj) => {
            tokens.next_token();
            return Node::Object {
                adjective: Box::new(Node::Value(adj)),
                noun: object_expr(tokens)?,
            }
            .into();
        }
        Token::Noun(noun) => {
            tokens.next_token();
            return Node::Value(noun).into();
        }
        _ => {
            return Err(format!(
                "Expected [Object] to take an action.\n <Sentence> -> <Verb> <Object>
 <Object> -> <Adjective> <Noun> | <Noun>"
            ))
        }
    };
}
