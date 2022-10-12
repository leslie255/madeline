pub mod fileformat;
pub mod generation;
pub mod ir;

#[cfg(test)]
mod tests {
    use crate::{
        fileformat::FileFormat,
        generation,
        ir::{
            DataType, Instruction, Operand, OperandContent as OperContent, OperationType, Program,
            TopLevelElement,
        },
    };

    macro_rules! s {
        ($s: expr) => {
            std::rc::Rc::new($s.to_string())
        };
    }

    macro_rules! i {
        ($opcode: tt, $oper0: expr, $oper1: expr) => {
            Instruction {
                operation: OperationType::$opcode,
                operand0: $oper0,
                operand1: $oper1,
            }
        };
    }

    macro_rules! o {
        ($t: tt, $content: tt, $value: tt) => {
            Operand {
                dtype: DataType::$t,
                content: OperContent::$content($value),
            }
        };
        () => {
            Operand::default()
        };
    }

    #[test]
    fn variables() {
        let program = Program {
            content: vec![
                TopLevelElement::DataStr("str".to_string(), "hello, world\n".to_string()),
                TopLevelElement::FnDef(
                    s!("main"),
                    vec![
                        i!(DefVar, o!(Signed64, Var, 0), o!()),
                        i!(SetVar, o!(Signed64, Var, 0), o!(Signed64, Data, 255)),
                        i!(RetVal, o!(Signed32, Data, 0), o!()),
                    ],
                ),
            ],
        };
        println!("------------- macho64 -------------");
        println!(
            "{}",
            generation::x86_64::generate_asm(program.clone(), FileFormat::Macho64)
        );
        println!("-------------- elf64 --------------");
        println!(
            "{}",
            generation::x86_64::generate_asm(program.clone(), FileFormat::Elf64)
        );
    }
}
