use super::super::fileformat::*;
use super::super::ir::*;

use std::collections::HashMap;

macro_rules! reg_name {
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
            4 => ", $d",
            2 => ", $w",
            1 => ", $b",
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
            "{}\tpop\trbp\n\tret\n\n",
            if $stack_depth != 0 {
                format!("\tadd\trsp, {}\n", $stack_depth)
            } else {
                format!("")
            }
        )
    }};
}

pub fn gen_instr(
    instr: &Instruction,
    fformat: FileFormat,
    var_addrs: &HashMap<String, u64>,
    stack_depth: u64,
) -> String {
    match &instr.operation {
        OperationType::SetVar => {
            let var_addr = format!(
                "[rbp - {}]",
                var_addrs
                    .get(instr.operand0.content.expect_var())
                    .expect("variable not defined (lhs)")
            );
            let mut code = String::new();
            match &instr.operand1.content {
                OperandContent::Data(i) => {
                    code.push_str(format!("\tmov\t{}, {}\n", var_addr, i,).as_str())
                }
                OperandContent::Var(var_name) => {
                    let rax = reg_name!(rax, instr.operand0.dtype.size());
                    code.push_str(
                        format!(
                            "\tmov\t{}, [rbp - {}]\n",
                            rax.clone(),
                            var_addrs.get(var_name).expect("variable not defined"),
                        )
                        .as_str(),
                    );
                    code.push_str(format!("\tmov\t{}, {}\n", var_addr, rax).as_str());
                }
                OperandContent::RetVal => code.push_str(
                    format!(
                        "\tmov\t{}, {}\n",
                        var_addr,
                        reg_name!(rax, instr.operand0.dtype.size())
                    )
                    .as_str(),
                ),
                _ => panic!("expects data, var, ret_val"),
            }
            code
        }
        OperationType::SetArg => todo!(),
        OperationType::CallFn => {
            format!(
                "\tcall\t{}\n",
                fformat.label(instr.operand0.content.expect_fn().clone())
            )
        }
        OperationType::Ret => {
            instr.operand1.content.expect_empty();
            let mut code = String::new();
            match &instr.operand0.content {
                OperandContent::Data(i) => code.push_str(
                    format!(
                        "\tmov\t{}, {}\n",
                        reg_name!(rax, instr.operand0.dtype.size()),
                        i,
                    )
                    .as_str(),
                ),
                OperandContent::Var(var_name) => code.push_str(
                    format!(
                        "\tmov\t{}, [rbp - {}]\n",
                        reg_name!(rax, instr.operand0.dtype.size()),
                        var_addrs.get(var_name).expect("variable not defined"),
                    )
                    .as_str(),
                ),
                OperandContent::RetVal => (),
                _ => panic!("expects data, var, ret_val"),
            }
            code.push_str(asm_code!(fn_epilog, stack_depth).as_str());
            code
        }
        OperationType::Add => todo!(),
        OperationType::Sub => todo!(),
        OperationType::Mul => todo!(),
        OperationType::Div => todo!(),
        OperationType::Inc => todo!(),
        OperationType::Dec => todo!(),
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
                    if !stack_depth.is_power_of_two() && stack_depth == 0 {
                        let mut i = 1u64;
                        while stack_depth < i {
                            i *= 2;
                            stack_depth = 1;
                        }
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
