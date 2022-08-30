pub mod ir;
use crate::ir::*;
pub mod tokens;
use crate::tokens::*;
pub mod generation;
use crate::generation::*;
pub mod fileformat;
use crate::fileformat::*;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let src_path = if let Some(p) = env::args().nth(1) {
        p
    } else {
        println!("expected 3 arguments");
        std::process::exit(1);
    };
    let fformat_str = if let Some(ff) = env::args().nth(2) {
        ff
    } else {
        println!("expected 3 arguments");
        std::process::exit(1);
    };
    let fformat =
        FileFormat::from_str(fformat_str).expect("expects `elf64` or `macho64` for file format");
    let output_path = if let Some(p) = env::args().nth(3) {
        p
    } else {
        println!("expected 3 arguments");
        std::process::exit(1);
    };
    let source = fs::read_to_string(src_path).unwrap();

    let token_stream = TokenStream::new(&source);
    let program = Program::parse_from(token_stream);
    program.print_code();
    let generated = x86_64::generate_asm(program, fformat);

    if Path::new(&output_path).exists() {
        fs::remove_file(output_path.clone()).unwrap_or_else(|_| {
            panic!(
                "Output file {} already exists and cannot be deleted",
                output_path
            )
        });
    }

    let mut output_file = fs::File::create(output_path.clone())
        .unwrap_or_else(|_| panic!("cannot write to file (0) {}", output_path));
    write!(output_file, "{}", generated)
        .unwrap_or_else(|_| panic!("cannot write to file (1) {}", output_path));
}
