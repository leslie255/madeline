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
fn x64_codegen() {
    use generation::x86_64;
    use generation::x86_64::Instruction as x64Instr;
    use generation::x86_64::Operand as x64Oper;
    use generation::x86_64::Register as x64Reg;
    use std::rc::Rc;
    macro_rules! rcstr {
        ($s: expr) => {
            Rc::new($s.to_string())
        };
    }

    let program = vec![
        x64Instr::GlobalLabel(rcstr!("main")),
        x64Instr::FnProlog,
        x64Instr::Mov(x64Oper::Reg(x64Reg::Rax), x64Oper::Im(0u64.to_be_bytes())),
        x64Instr::FnEpilog,
    ];
    let mut generated_asm = String::new();
    x86_64::gen_asm_from_model(fileformat::FileFormat::Elf64, program, &mut generated_asm).unwrap();
    println!("{generated_asm}");
}
