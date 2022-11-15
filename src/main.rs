use std::{env, fs::read_to_string};

pub mod fileformat;
pub mod generation;
pub mod ir;
mod parser;

fn main() {
    let mut args = env::args();
    args.next();
    let src_path = args
        .next()
        .expect("Expect one argument for the source file path");
    let src_content = read_to_string(src_path).expect("Enable to read file into string");
    let tokens = parser::parse_string_into_tokens(src_content);
    let ir_program = parser::parse_tokens_into_ir(tokens);
    let code = generation::x86_64::gen_code(ir_program);
    let mut generated_asm = String::new();
    generation::x86_64::gen_asm_from_model(
        fileformat::FileFormat::Macho64,
        code,
        &mut generated_asm,
    )
    .unwrap();
    print!("{generated_asm}");
}
