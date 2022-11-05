#![allow(unused)]

use std::{
    fmt::{Display, Write},
    rc::Rc,
};

use super::virt_reg::{Register, VRegAllocator, VRegContentKind};
use crate::{
    fileformat::FileFormat,
    ir::{DataType, Instruction as IRInstruction, TopLevel as IRTopLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum X86WordSize {
    Byte = 1,
    Word = 2,
    Bword = 4,
    Qword = 8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    GlobalLabel(Rc<String>),
    Label(Rc<String>),
    FnProlog,
    FnEpilog,
    Ret,
    AllocStack(u64),
    DeallocStack(u64),

    Mov(Operand, Operand),
    Movzx(Operand, Operand),
    Lea(Operand, Operand),

    Call(Rc<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Reg(X64Register),
    Im([u8; 8]),
    Label(Rc<String>),
    Load(EvalTreeNode),                 // [ ... ]
    WordPtr(X86WordSize, EvalTreeNode), // qword [ ... ]
}
impl Operand {
    pub fn gen_code(&self, file_format: FileFormat) -> Result<String, std::fmt::Error> {
        let mut code = String::new();
        match self {
            Self::Reg(reg) => write!(code, "{}", reg)?,
            Self::Im(bytes) => write!(code, "{}", u64::from_be_bytes(bytes.clone()))?,
            Self::Label(name) => write!(code, "{}", file_format.mangle(name))?,
            Self::Load(eval_tree) => write!(code, "[{}]", eval_tree)?,
            Self::WordPtr(size, eval_tree) => {
                write!(code, "{} [{}]", size.fmt_into_asm(), eval_tree)?
            }
        }
        Ok(code)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalTreeNode {
    // [ ... ] in asm
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),

    Num(u64),
    Reg(X64Register),
}
impl EvalTreeNode {
    pub fn priority(&self) -> usize {
        match self {
            Self::Add(_, _) => 0,
            Self::Sub(_, _) => 0,
            Self::Mul(_, _) => 1,
            Self::Num(_) => 2,
            Self::Reg(_) => 2,
        }
    }
    pub fn op_char(&self) -> char {
        match self {
            Self::Add(_, _) => '+',
            Self::Sub(_, _) => '-',
            Self::Mul(_, _) => '*',
            Self::Num(_) => '\0',
            Self::Reg(_) => '\0',
        }
    }
}
impl Display for EvalTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add(lhs, rhs) | Self::Sub(lhs, rhs) | Self::Mul(lhs, rhs) => {
                if lhs.priority() < self.priority() {
                    write!(f, "({})", lhs)?;
                } else {
                    lhs.fmt(f)?;
                }
                self.op_char().fmt(f)?;
                if rhs.priority() < self.priority() {
                    write!(f, "({})", rhs)?;
                } else {
                    rhs.fmt(f)?;
                }
            }
            Self::Num(num) => {
                num.fmt(f)?;
            }
            Self::Reg(reg) => {
                reg.fmt(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X64Register {
    Rax = 0x00,
    Rbx,
    Rcx,
    Rdx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    Eax = 0x10,
    Ebx,
    Ecx,
    Edx,
    Esp,
    Ebp,
    Esi,
    Edi,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,

    Ax = 0x20,
    Bx,
    Cx,
    Dx,
    Sp,
    Bp,
    Si,
    Di,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,

    Al = 0x30,
    Bl,
    Cl,
    Dl,
    Spl,
    Bpl,
    Sil,
    Dil,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

impl Display for X64Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X64Register::Rax => write!(f, "rax")?,
            X64Register::Rbx => write!(f, "rbx")?,
            X64Register::Rcx => write!(f, "rcx")?,
            X64Register::Rdx => write!(f, "rdx")?,
            X64Register::Rsp => write!(f, "rsp")?,
            X64Register::Rbp => write!(f, "rbp")?,
            X64Register::Rsi => write!(f, "rsi")?,
            X64Register::Rdi => write!(f, "rdi")?,
            X64Register::R8 => write!(f, "r8")?,
            X64Register::R9 => write!(f, "r9")?,
            X64Register::R10 => write!(f, "r10")?,
            X64Register::R11 => write!(f, "r11")?,
            X64Register::R12 => write!(f, "r12")?,
            X64Register::R13 => write!(f, "r13")?,
            X64Register::R14 => write!(f, "r14")?,
            X64Register::R15 => write!(f, "r15")?,
            X64Register::Eax => write!(f, "eax")?,
            X64Register::Ebx => write!(f, "ebx")?,
            X64Register::Ecx => write!(f, "ecx")?,
            X64Register::Edx => write!(f, "edx")?,
            X64Register::Esp => write!(f, "esp")?,
            X64Register::Ebp => write!(f, "ebp")?,
            X64Register::Esi => write!(f, "esi")?,
            X64Register::Edi => write!(f, "edi")?,
            X64Register::R8d => write!(f, "r8d")?,
            X64Register::R9d => write!(f, "r9d")?,
            X64Register::R10d => write!(f, "r10d")?,
            X64Register::R11d => write!(f, "r11d")?,
            X64Register::R12d => write!(f, "r12d")?,
            X64Register::R13d => write!(f, "r13d")?,
            X64Register::R14d => write!(f, "r14d")?,
            X64Register::R15d => write!(f, "r15d")?,
            X64Register::Ax => write!(f, "ax")?,
            X64Register::Bx => write!(f, "bx")?,
            X64Register::Cx => write!(f, "cx")?,
            X64Register::Dx => write!(f, "dx")?,
            X64Register::Sp => write!(f, "sp")?,
            X64Register::Bp => write!(f, "bp")?,
            X64Register::Si => write!(f, "si")?,
            X64Register::Di => write!(f, "di")?,
            X64Register::R8w => write!(f, "r8w")?,
            X64Register::R9w => write!(f, "r9w")?,
            X64Register::R10w => write!(f, "r10w")?,
            X64Register::R11w => write!(f, "r11w")?,
            X64Register::R12w => write!(f, "r12w")?,
            X64Register::R13w => write!(f, "r13w")?,
            X64Register::R14w => write!(f, "r14w")?,
            X64Register::R15w => write!(f, "r15w")?,
            X64Register::Al => write!(f, "al")?,
            X64Register::Bl => write!(f, "bl")?,
            X64Register::Cl => write!(f, "cl")?,
            X64Register::Dl => write!(f, "dl")?,
            X64Register::Spl => write!(f, "spl")?,
            X64Register::Bpl => write!(f, "bpl")?,
            X64Register::Sil => write!(f, "sil")?,
            X64Register::Dil => write!(f, "dil")?,
            X64Register::R8b => write!(f, "r8b")?,
            X64Register::R9b => write!(f, "r9b")?,
            X64Register::R10b => write!(f, "r10b")?,
            X64Register::R11b => write!(f, "r11b")?,
            X64Register::R12b => write!(f, "r12b")?,
            X64Register::R13b => write!(f, "r13b")?,
            X64Register::R14b => write!(f, "r14b")?,
            X64Register::R15b => write!(f, "r15b")?,
        }
        Ok(())
    }
}

impl X64Register {
    pub fn from_raw(raw: usize) -> Self {
        unsafe {
            let ptr = &raw as *const usize;
            return *(ptr as *const Self);
        }
    }
}
impl X64Register {
    fn of_size(self, size: X86WordSize) -> Self {
        let mut raw = self as usize;
        raw &= 0x0F;
        match size {
            X86WordSize::Byte => raw += 0x30,
            X86WordSize::Word => raw += 0x20,
            X86WordSize::Bword => raw += 0x10,
            X86WordSize::Qword => (),
        }
        Self::from_raw(raw)
    }
    fn word_size(self) -> X86WordSize {
        match (self as usize) & 0xF0 {
            0x00 => X86WordSize::Qword,
            0x10 => X86WordSize::Bword,
            0x20 => X86WordSize::Word,
            0x30 => X86WordSize::Byte,
            _ => panic!(),
        }
    }
}
impl Register for X64Register {
    fn caller_saved() -> Vec<Self> {
        vec![
            Self::Rdi,
            Self::Rsi,
            Self::Rdx,
            Self::Rcx,
            Self::R8,
            Self::R9,
        ]
    }
    fn callee_saved() -> Vec<Self> {
        vec![
            Self::Rbx,
            Self::Rsp,
            Self::Rbp,
            Self::R10,
            Self::R11,
            Self::R12,
            Self::R13,
            Self::R14,
            Self::R15,
        ]
    }
}
trait FmtIntoX64Asm {
    fn fmt_into_asm(&self) -> String;
}
impl FmtIntoX64Asm for X86WordSize {
    fn fmt_into_asm(&self) -> String {
        let mut str = String::with_capacity(5);
        match self {
            X86WordSize::Byte => write!(str, "byte"),
            X86WordSize::Word => write!(str, "word"),
            X86WordSize::Bword => write!(str, "bword"),
            X86WordSize::Qword => write!(str, "qword"),
        }
        .unwrap();
        str
    }
}

pub fn gen_asm_from_model(
    file_format: FileFormat,
    instructions: Vec<Instruction>,
    target: &mut dyn Write,
) -> Result<(), std::fmt::Error> {
    for instruction in instructions {
        match instruction {
            Instruction::GlobalLabel(name) => writeln!(
                target,
                "\tglobal\t{}\n{}:",
                file_format.mangle(&name),
                file_format.mangle(&name)
            )?,
            Instruction::Label(name) => writeln!(target, "{}:", file_format.mangle(&name))?,
            Instruction::FnProlog => writeln!(target, "\tpush\trbp\n\tmov\trbp, rsp")?,
            Instruction::FnEpilog => writeln!(target, "\tpop\trbp\n\tret")?,
            Instruction::Ret => writeln!(target, "\tret")?,
            Instruction::AllocStack(depth) => writeln!(target, "\tsub\trsp, {}", depth)?,
            Instruction::DeallocStack(depth) => writeln!(target, "\tadd\trsp, {}", depth)?,
            Instruction::Mov(oper0, oper1) => match (oper0, oper1) {
                (Operand::Reg(reg), Operand::Im([0, 0, 0, 0, 0, 0, 0, 0])) => {
                    writeln!(target, "\txor\t{}, {}", reg, reg)?
                }
                (oper0, oper1) => writeln!(
                    target,
                    "\tmov\t{}, {}",
                    oper0.gen_code(file_format)?,
                    oper1.gen_code(file_format)?,
                )?,
            },
            Instruction::Movzx(oper0, oper1) => writeln!(
                target,
                "\tmovzx\t{}, {}",
                oper0.gen_code(file_format)?,
                oper1.gen_code(file_format)?
            )?,
            Instruction::Lea(oper0, oper1) => writeln!(
                target,
                "\tlea\t{}, {}",
                oper0.gen_code(file_format)?,
                oper1.gen_code(file_format)?
            )?,
            Instruction::Call(name) => writeln!(target, "call\t{}", file_format.mangle(&name))?,
        }
    }
    Ok(())
}

pub fn gen_code(ir: Vec<IRTopLevel>) -> Vec<Instruction> {
    let mut generated = Vec::<Instruction>::new();
    for ir_top_level in ir {
        match ir_top_level {
            IRTopLevel::Extern(_) => todo!(),
            IRTopLevel::Fn { name, args, body } => gen_inside_fn(name, args, body, &mut generated),
        }
    }
    generated
}

fn gen_inside_fn(
    name: Rc<String>,
    args: Vec<DataType>,
    body: Vec<IRInstruction>,
    target: &mut Vec<Instruction>,
) {
    let vreg_count = body.iter().filter(|&i| i.is_def_reg()).count();
    let step_count = body.len();
    let mut reg_map = VRegAllocator::<X64Register>::empty(step_count, vreg_count);
    reg_map.generate_map_from(&body);
    reg_map.alloc_regs();

    reg_map.print_reg_lifetime_map();
    reg_map.print_reg_infos();

    target.push(Instruction::GlobalLabel(name));
    target.push(Instruction::FnProlog);
    target.push(Instruction::FnEpilog);
    target.push(Instruction::Ret);
}
