use std::{env, fs::read_to_string};

pub mod fileformat;
pub mod generation;
pub mod ir;
mod parser;

use generation::platform;

fn main() {
    let mut args = env::args().skip(1);
    let src_path = args
        .next()
        .expect("Expect one argument for the source file path");
    let out_path = args
        .next()
        .expect("Expect one argument for the source file path");
    let src_content = read_to_string(src_path).expect("Enable to read file into string");
    let tokens = parser::parse_string_into_tokens(src_content);
    let ir_program = parser::parse_tokens_into_ir(tokens);
    println!("{ir_program:#?}");
    let code = platform::x86_64::gen_code(ir_program);
    let mut generated_asm = String::new();
    platform::x86_64::gen_asm_from_model(
        fileformat::FileFormat::Macho64,
        code,
        &mut generated_asm,
    )
    .unwrap();
    std::fs::write(out_path.clone(), generated_asm).expect("Unable to write to output path");
    println!("Output written to {:?}", out_path);
}
