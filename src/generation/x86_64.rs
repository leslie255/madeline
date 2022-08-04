use super::super::fileformat::*;
use super::super::ir::*;

use std::collections::HashMap;

macro_rules! reg_name {
    (convert, $name: expr, $len: expr) => {
        String::from(match ($name, $len) {
            ("rax", 8) => "rax",
            ("rax", 4) => "eax",
            ("rax", 2) => "ax",
            ("rax", 1) => "al",
            ("rbx", 8) => "rbx",
            ("rbx", 4) => "ebx",
            ("rbx", 2) => "bx",
            ("rbx", 1) => "bl",
            ("rcx", 8) => "rcx",
            ("rcx", 4) => "ecx",
            ("rcx", 2) => "cx",
            ("rcx", 1) => "cl",
            ("rdx", 8) => "rdx",
            ("rdx", 4) => "edx",
            ("rdx", 2) => "dx",
            ("rdx", 1) => "dl",
            ("rsi", 8) => "rsi",
            ("rsi", 4) => "esi",
            ("rsi", 2) => "si",
            ("rsi", 1) => "sil",
            ("rdi", 8) => "rdi",
            ("rdi", 4) => "edi",
            ("rdi", 2) => "di",
            ("rdi", 1) => "dil",
            ("rsp", 8) => "rsp",
            ("rsp", 4) => "esp",
            ("rsp", 2) => "sp",
            ("rsp", 1) => "spl",
            ("rbp", 8) => "rbp",
            ("rbp", 4) => "ebp",
            ("rbp", 2) => "bp",
            ("rbp", 1) => "bpl",
            ("r8", 8) => "r8",
            ("r8", 4) => "r8d",
            ("r8", 2) => "r8w",
            ("r8", 1) => "r8b",
            ("r9", 8) => "r9",
            ("r9", 4) => "r9d",
            ("r9", 2) => "r9w",
            ("r9", 1) => "r9b",
            ("r10", 8) => "r10",
            ("r10", 4) => "r10d",
            ("r10", 2) => "r10w",
            ("r10", 1) => "r10b",
            ("r11", 8) => "r11",
            ("r11", 4) => "r11d",
            ("r11", 2) => "r11w",
            ("r11", 1) => "r11b",
            ("r12", 8) => "r12",
            ("r12", 4) => "r12d",
            ("r12", 2) => "r12w",
            ("r12", 1) => "r12b",
            ("r13", 8) => "r13",
            ("r13", 4) => "r13d",
            ("r13", 2) => "r13w",
            ("r13", 1) => "r13b",
            ("r14", 8) => "r14",
            ("r14", 4) => "r14d",
            ("r14", 2) => "r14w",
            ("r14", 1) => "r14b",
            ("r15", 8) => "r15",
            ("r15", 4) => "r15d",
            ("r15", 2) => "r15w",
            ("r15", 1) => "r15b",
            _ => panic!(),
        })
    };
    (rax, $len: expr) => {
        String::from(match $len {
            8 => "rax",
            4 => "eax",
            2 => "ax",
            1 => "al",
            _ => panic!(),
        })
    };
    (rbx, $len: expr) => {
        String::from(match $len {
            8 => "rbx",
            4 => "ebx",
            2 => "bx",
            1 => "bl",
            _ => panic!(),
        })
    };
    (rcx, $len: expr) => {
        String::from(match $len {
            8 => "rcx",
            4 => "ecx",
            2 => "cx",
            1 => "cl",
            _ => panic!(),
        })
    };
    (rdx, $len: expr) => {
        String::from(match $len {
            8 => "rdx",
            4 => "edx",
            2 => "dx",
            1 => "dl",
            _ => panic!(),
        })
    };
    (rsi, $len: expr) => {
        String::from(match $len {
            8 => "rsi",
            4 => "esi",
            2 => "si",
            1 => "sil",
            _ => panic!(),
        })
    };
    (rdi, $len: expr) => {
        String::from(match $len {
            8 => "rdi",
            4 => "edi",
            2 => "di",
            1 => "dil",
            _ => panic!(),
        })
    };
    (rsp, $len: expr) => {
        String::from(match $len {
            8 => "rsp",
            4 => "esp",
            2 => "sp",
            1 => "spl",
            _ => panic!(),
        })
    };
    (rbp, $len: expr) => {
        String::from(match $len {
            8 => "rbp",
            4 => "ebp",
            2 => "bp",
            1 => "bpl",
            _ => panic!(),
        })
    };
    (r8, $len: expr) => {
        String::from(match $len {
            8 => "r8",
            4 => "r8d",
            2 => "r8w",
            1 => "r8b",
            _ => panic!(),
        })
    };
    (r9, $len: expr) => {
        String::from(match $len {
            8 => "r9",
            4 => "r9d",
            2 => "r9w",
            1 => "r9b",
            _ => panic!(),
        })
    };
    (r10, $len: expr) => {
        String::from(match $len {
            8 => "r10",
            4 => "r10d",
            2 => "r10w",
            1 => "r10b",
            _ => panic!(),
        })
    };
    (r11, $len: expr) => {
        String::from(match $len {
            8 => "r11",
            4 => "r11d",
            2 => "r11w",
            1 => "r11b",
            _ => panic!(),
        })
    };
    (r12, $len: expr) => {
        String::from(match $len {
            8 => "r12",
            4 => "r12d",
            2 => "r12w",
            1 => "r12b",
            _ => panic!(),
        })
    };
    (r13, $len: expr) => {
        String::from(match $len {
            8 => "r13",
            4 => "r13d",
            2 => "r13w",
            1 => "r13b",
            _ => panic!(),
        })
    };
    (r14, $len: expr) => {
        String::from(match $len {
            8 => "r14",
            4 => "r14d",
            2 => "r14w",
            1 => "r14b",
            _ => panic!(),
        })
    };
    (r15, $len: expr) => {
        String::from(match $len {
            8 => "r15",
            4 => "r15d",
            2 => "r15w",
            1 => "r15b",
            _ => panic!(),
        })
    };
}

macro_rules! asm_code {
    (fn_prolog, $fformat: expr, $fn_name: expr, $stack_depth: expr) => {{
        let fn_label_name = $fformat.label($fn_name);
        format!(
            "\tglobal\t{}\n{}:\n\tpush\trbp\n\tmov\trbp, rsp\n{}\n",
            fn_label_name,
            fn_label_name,
            if $stack_depth != 0 {
                format!("\tsub\trsp, {}\n", $stack_depth)
            } else {
                format!("")
            }
        )
    }};
    (fn_epilog, $stack_depth: expr) => {{
        format!(
            "{}\tpop\trbp\n\tret\n",
            if $stack_depth != 0 {
                format!("\tadd\trsp, {}\n", $stack_depth)
            } else {
                format!("")
            }
        )
    }};
}

static ARG_REGS: [&'static str; 6] = ["rsi", "rdi", "rdx", "rcx", "r8", "r9"];

fn asm_for_operand(operand: &Operand, var_addrs: &HashMap<String, u64>) -> String {
    match &operand.content {
        OperandContent::Data(i) => format!("{}", i),
        OperandContent::Var(var_name) => {
            let var_addr = var_addrs.get(var_name).expect("undefined variable");
            format!("[rbp - {}]", var_addr)
        }
        OperandContent::Arg(arg_i) => {
            reg_name!(convert, ARG_REGS[*arg_i as usize], operand.dtype.size())
        }
        OperandContent::RetVal => reg_name!(rax, operand.dtype.size()),
        _ => panic!("expects data, var, arg, ret_val"),
    }
}

fn move_instr(lhs: &Operand, rhs: &Operand, var_addrs: &HashMap<String, u64>) -> String {
    let lhs_addr = var_addrs
        .get(lhs.content.expect_var())
        .expect("undefined variable");
    match &rhs.content {
        OperandContent::Var(rhs_name) => {
            let rax = reg_name!(rax, lhs.dtype.size());
            let rhs_addr = var_addrs.get(rhs_name).expect("undefined variable");
            format!(
                "\tmov\t{}, [rbp - {}]\n\tmov\t[rbp - {}], {}\n",
                rax, rhs_addr, lhs_addr, rax
            )
        }
        _ => {
            format!(
                "\tmov\t[rbp - {}], {}\n",
                lhs_addr,
                asm_for_operand(rhs, var_addrs)
            )
        }
    }
}

fn move_to_reg(reg: &String, rhs: &Operand, var_addrs: &HashMap<String, u64>) -> String {
    match &rhs.content {
        OperandContent::Data(0) => {
            format!("\txor\t{}, {}\n", reg, reg)
        }
        _ => format!("\tmov\t{}, {}\n", reg, asm_for_operand(rhs, var_addrs)),
    }
}

pub fn gen_instr(
    instr: &Instruction,
    fformat: FileFormat,
    var_addrs: &HashMap<String, u64>,
    stack_depth: u64,
) -> String {
    macro_rules! op_prefix {
        // returns "i" if the data type is signed
        // "f" is is floating point types
        // "" if neither
        () => {
            match &instr.operand0.dtype {
                DataType::Unsigned64
                | DataType::Unsigned32
                | DataType::Unsigned16
                | DataType::Unsigned8 => "",
                DataType::Signed64
                | DataType::Signed32
                | DataType::Signed16
                | DataType::Signed8 => "i",
                DataType::Float64 | DataType::Float32 => "f",
                DataType::Irrelavent => "",
            }
        };
    }
    match &instr.operation {
        OperationType::SetVar => move_instr(&instr.operand0, &instr.operand1, &var_addrs),
        OperationType::SetArg => {
            let arg_i = *instr.operand0.content.expect_arg();
            if arg_i >= 6 {
                panic!("passing more than 6 arugments into a function hasn't been implemented yet");
            }
            let arg_reg = reg_name!(
                convert,
                ARG_REGS[arg_i as usize],
                instr.operand0.dtype.size()
            );
            move_to_reg(&arg_reg, &instr.operand1, &var_addrs)
        }
        OperationType::CallFn => {
            format!(
                "\tcall\t{}\n",
                fformat.label(instr.operand0.content.expect_fn().clone())
            )
        }
        OperationType::Ret => {
            instr.operand1.content.expect_empty();
            let mut code = String::from("\n");
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            if !instr.operand0.is_irrelavent() {
                code.push_str(&move_to_reg(&rax, &instr.operand0, &var_addrs));
            }
            code.push_str(asm_code!(fn_epilog, stack_depth).as_str());
            code
        }
        OperationType::Add => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\t{}add\t{}, {}\n",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                op_prefix!(),
                rax,
                asm_for_operand(&instr.operand1, var_addrs),
            )
        }
        OperationType::Sub => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\t{}sub\t{}, {}\n",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                op_prefix!(),
                rax,
                asm_for_operand(&instr.operand1, var_addrs),
            )
        }
        OperationType::Mul => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\t{}mul\t{}\n",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                op_prefix!(),
                asm_for_operand(&instr.operand1, var_addrs),
            )
        }
        OperationType::Div => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\t{}div\t{}\n\t",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                op_prefix!(),
                asm_for_operand(&instr.operand1, var_addrs),
            )
        }
        OperationType::RawASM => {
            format!("\t{}\n", instr.operand0.content.expect_raw_asm())
        }
        OperationType::Inc => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\tinc\t{}\n\tmov\t{}, {}\n",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                rax,
            )
        }
        OperationType::Dec => {
            let rax = reg_name!(rax, instr.operand0.dtype.size());
            format!(
                "\tmov\t{}, {}\n\tdec\t{}\n\tmov\t{}, {}\n",
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                rax,
                asm_for_operand(&instr.operand0, var_addrs),
                rax,
            )
        }
        OperationType::BlockStart => todo!(),
        OperationType::Label => format!(
            "{}:\n",
            fformat.label(instr.operand0.content.expect_label().clone())
        ),
        OperationType::Jmp => format!(
            "\tjmp\t{}\n",
            fformat.label(instr.operand0.content.expect_label().clone())
        ),
        OperationType::Cmp => match (&instr.operand0.content, &instr.operand1.content) {
            (OperandContent::Var(..), OperandContent::Var(..)) => {
                let var2_addr = asm_for_operand(&instr.operand1, var_addrs);
                let rax = reg_name!(rax, instr.operand0.dtype.size());
                format!(
                    "\tmov\t{}, {}\n\tcmp\t{}, {}\n",
                    rax,
                    var2_addr,
                    rax,
                    asm_for_operand(&instr.operand0, var_addrs),
                )
            }
            _ => {
                format!(
                    "\tcmp\t{}, {}\n",
                    asm_for_operand(&instr.operand0, var_addrs),
                    asm_for_operand(&instr.operand1, var_addrs)
                )
            }
        },
        OperationType::Je => todo!(),
        OperationType::Jn => todo!(),
        OperationType::Jz => todo!(),
        OperationType::Jnz => todo!(),
        OperationType::Jg => todo!(),
        OperationType::Jl => todo!(),
        OperationType::Jnge => todo!(),
        OperationType::Jnle => todo!(),
    }
}

pub fn generate_asm(program: Program, fformat: FileFormat) -> String {
    let mut code = String::new();
    for top_level_expr in program.content {
        match top_level_expr {
            TopLevelElement::FnDef(fn_name, vars, body) => {
                let mut stack_depth: u64;
                let mut var_addrs: HashMap<String, u64> = HashMap::new();
                if !vars.is_empty() {
                    stack_depth = 8;
                    for (var_name, dtype) in vars {
                        var_addrs.insert(var_name, stack_depth);
                        stack_depth += dtype.size();
                    }
                    if !stack_depth.is_power_of_two() {
                        let mut i = 1u64;
                        while stack_depth > i {
                            i *= 2;
                        }
                        stack_depth = i;
                    }
                } else {
                    stack_depth = 0;
                }
                code.push_str(&asm_code!(fn_prolog, fformat, fn_name, stack_depth));
                for instr in &body {
                    code.push_str(&gen_instr(instr, fformat, &var_addrs, stack_depth));
                }
            }
        }
    }
    return code;
}
