pub mod ir;
use crate::ir::*;
pub mod tokens;
use crate::tokens::*;

static SOURCE: &str = include_str!("../test.mir");

fn main() {
    let source = String::from(SOURCE);
    let mut token_stream = TokenStream::new(&source);
    loop {
        if let Some(token) = token_stream.next() {
            println!("{:?}", token);
        } else {
            break;
        }
    }
}
