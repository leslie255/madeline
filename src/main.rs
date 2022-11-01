use std::{env, fs::read_to_string};

pub mod fileformat;
pub mod generation;
pub mod ir;
mod tokens;

fn main() {
    let mut args = env::args();
    args.next();
    let src_path = args.next().expect("Expect one argument for the source file path");
    let src_content = read_to_string(src_path).expect("Enable to read file into string");
    let tokens = tokens::parse_into_tokens(src_content);
    tokens.iter().for_each(|t|println!("{t:?}"));
}
