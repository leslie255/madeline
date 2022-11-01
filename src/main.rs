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
    tokens.iter().for_each(|t| println!("{t:?}"));
    println!("\n");
    let program = parser::parse_tokens_into_ir(tokens);
    program.iter().for_each(|i| println!("{i:#?}"));
}

#[test]
fn parse_tokens() {
    use parser::*;
    let src = r#"
        "\"A\n\x42"
        fn @main(u32, ptr) {
            ret i32 $0
        }
        "#
    .to_string();
    let tokens = parse_string_into_tokens(src);
    println!("{tokens:?}");
}
