pub mod ir;
use crate::ir::*;
pub mod tokens;
use crate::tokens::*;

fn main() {
    let src_path = if let Some(p) = std::env::args().nth(1) {
        p
    } else {
        println!("expected one argument for file path, found zero");
        std::process::exit(1);
    };
    let source = std::fs::read_to_string(src_path).unwrap();

    let token_stream = TokenStream::new(&source);
    let program = Program::parse_from(token_stream);
}
