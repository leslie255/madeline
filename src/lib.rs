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
        ($t: tt, $content: tt, $value: expr) => {
            Operand {
                dtype: DataType::$t,
                content: OperContent::$content($value),
            }
        };
        ($t: tt, $content: tt) => {
            Operand {
                dtype: DataType::$t,
                content: OperContent::$content,
            }
        };
        ($t: tt) => {
            Operand {
                dtype: DataType::$t,
                content: OperContent::Irrelavent,
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
                        i!(DefVar, o!(SignedSize, Var, 0), o!()),
                        i!(SetVar, o!(SignedSize, Var, 0), o!(Unsigned64, Data, 255)),
                        i!(DefVar, o!(SignedSize, Var, 1), o!()),
                        i!(RetVal, o!(Unsigned32, Data, 0), o!()),
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

    #[test]
    fn deref() {
        let program = Program {
            content: vec![
                TopLevelElement::Extern(s!("printf")),
                TopLevelElement::DataStr("fmt".to_string(), "%llu\n".to_string()),
                TopLevelElement::FnDef(
                    s!("main"),
                    vec![
                        i!(DefVar, o!(Unsigned64, Var, 0), o!()),
                        i!(DefVar, o!(Pointer, Var, 1), o!()),
                        i!(DefVar, o!(Unsigned64, Var, 2), o!()),
                        i!(SetVar, o!(Unsigned64, Var, 0), o!(Unsigned64, Data, 42)),
                        // %1 = addr(%0);
                        i!(TakeAddr, o!(Unsigned64, Var, 0), o!()),
                        i!(SetVar, o!(Pointer, Var, 1), o!(Pointer, Result)),
                        // %2 = [%1]
                        i!(Deref, o!(Pointer, Var, 1), o!(Unsigned64)),
                        i!(SetVar, o!(Unsigned64, Var, 2), o!(Unsigned64, Result)),
                        // printf("%llu\n", %2);
                        i!(SetArg, o!(Pointer, Arg, 0), o!(Pointer, Label, s!("fmt"))),
                        i!(SetArg, o!(Pointer, Arg, 1), o!(Unsigned64, Var, 2)),
                        i!(CallFn, o!(Irrelavent, Fn, s!("printf")), o!()),
                        i!(RetVal, o!(Unsigned32, Data, 0), o!()),
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
