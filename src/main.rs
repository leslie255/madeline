pub mod ir;
use crate::ir::*;
pub mod tokens;
use crate::tokens::*;
pub mod generation;
use crate::generation::*;
pub mod fileformat;
use crate::fileformat::*;

fn main() {
    let src_path = if let Some(p) = std::env::args().nth(1) {
        p
    } else {
        println!("expected two arguments");
        std::process::exit(1);
    };
    let fformat_str = if let Some(ff) = std::env::args().nth(2) {
        ff
    } else {
        println!("expected two arguments");
        std::process::exit(1);
    };
    let fformat =
        FileFormat::from_str(fformat_str).expect("expects `elf64` or `macho64` for file format");
    let source = std::fs::read_to_string(src_path).unwrap();

    let token_stream = TokenStream::new(&source);
    let program = Program::parse_from(token_stream);
    let generated = x86_64::generate_asm(program, fformat);
    print!("generated:\n---------------------------\n{}", generated);
}
