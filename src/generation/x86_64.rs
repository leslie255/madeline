#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    rc::Rc,
};

use crate::fileformat::FileFormat;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    GlobalLabel(Rc<String>),
    Label(Rc<String>),
    FnProlog,
    FnEpilog,
    AllocStack(u64),
    DeallocStack(u64),

    Mov(Operand, Operand),
    Call(Rc<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Reg(Register),
    Im([u8; 8]),
    Label(Rc<String>),
}
impl Operand {
    fn is_im_zero(&self) -> bool {
        if let Self::Im(bytes) = self {
            bytes.iter().find(|b| **b == 0).is_some()
        } else {
            false
        }
    }
}
impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg)?,
            Operand::Im(bytes) => write!(f, "{}", u64::from_be_bytes(bytes.clone()))?,
            Operand::Label(name) => write!(f, "{}:", name)?,
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Rax = 0x00,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
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
    Esi,
    Edi,
    Esp,
    Ebp,
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
    Si,
    Di,
    Sp,
    Bp,
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
    Sil,
    Dil,
    Spl,
    Bpl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Rax => write!(f, "rax")?,
            Register::Rbx => write!(f, "rbx")?,
            Register::Rcx => write!(f, "rcx")?,
            Register::Rdx => write!(f, "rdx")?,
            Register::Rsi => write!(f, "rsi")?,
            Register::Rdi => write!(f, "rdi")?,
            Register::Rsp => write!(f, "rsp")?,
            Register::Rbp => write!(f, "rbp")?,
            Register::R8 => write!(f, "r8")?,
            Register::R9 => write!(f, "r9")?,
            Register::R10 => write!(f, "r10")?,
            Register::R11 => write!(f, "r11")?,
            Register::R12 => write!(f, "r12")?,
            Register::R13 => write!(f, "r13")?,
            Register::R14 => write!(f, "r14")?,
            Register::R15 => write!(f, "r15")?,
            Register::Eax => write!(f, "eax")?,
            Register::Ebx => write!(f, "ebx")?,
            Register::Ecx => write!(f, "ecx")?,
            Register::Edx => write!(f, "edx")?,
            Register::Esi => write!(f, "esi")?,
            Register::Edi => write!(f, "edi")?,
            Register::Esp => write!(f, "esp")?,
            Register::Ebp => write!(f, "ebp")?,
            Register::R8d => write!(f, "r8d")?,
            Register::R9d => write!(f, "r9d")?,
            Register::R10d => write!(f, "r10d")?,
            Register::R11d => write!(f, "r11d")?,
            Register::R12d => write!(f, "r12d")?,
            Register::R13d => write!(f, "r13d")?,
            Register::R14d => write!(f, "r14d")?,
            Register::R15d => write!(f, "r15d")?,
            Register::Ax => write!(f, "ax")?,
            Register::Bx => write!(f, "bx")?,
            Register::Cx => write!(f, "cx")?,
            Register::Dx => write!(f, "dx")?,
            Register::Si => write!(f, "si")?,
            Register::Di => write!(f, "di")?,
            Register::Sp => write!(f, "sp")?,
            Register::Bp => write!(f, "bp")?,
            Register::R8w => write!(f, "r8w")?,
            Register::R9w => write!(f, "r9w")?,
            Register::R10w => write!(f, "r10w")?,
            Register::R11w => write!(f, "r11w")?,
            Register::R12w => write!(f, "r12w")?,
            Register::R13w => write!(f, "r13w")?,
            Register::R14w => write!(f, "r14w")?,
            Register::R15w => write!(f, "r15w")?,
            Register::Al => write!(f, "al")?,
            Register::Bl => write!(f, "bl")?,
            Register::Cl => write!(f, "cl")?,
            Register::Dl => write!(f, "dl")?,
            Register::Sil => write!(f, "sil")?,
            Register::Dil => write!(f, "dil")?,
            Register::Spl => write!(f, "spl")?,
            Register::Bpl => write!(f, "bpl")?,
            Register::R8b => write!(f, "r8b")?,
            Register::R9b => write!(f, "r9b")?,
            Register::R10b => write!(f, "r10b")?,
            Register::R11b => write!(f, "r11b")?,
            Register::R12b => write!(f, "r12b")?,
            Register::R13b => write!(f, "r13b")?,
            Register::R14b => write!(f, "r14b")?,
            Register::R15b => write!(f, "r15b")?,
        }
        Ok(())
    }
}

impl Register {
    pub fn from_raw(raw: usize) -> Self {
        unsafe {
            let ptr = &raw as *const usize;
            return *(ptr as *const Self);
        }
    }
    pub fn of_size(self, size: WordSize) -> Self {
        let mut raw = self as usize;
        raw &= 0x0F;
        match size {
            WordSize::Byte => raw += 0x30,
            WordSize::Word => raw += 0x20,
            WordSize::Bword => raw += 0x10,
            WordSize::Qword => (),
        }
        Self::from_raw(raw)
    }
    pub fn word_size(self) -> WordSize {
        let raw = self as usize;
        match raw & 0xF0 {
            0x00 => WordSize::Qword,
            0x10 => WordSize::Bword,
            0x20 => WordSize::Word,
            0x30 => WordSize::Byte,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WordSize {
    Byte = 1,
    Word = 2,
    Bword = 4,
    Qword = 8,
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
            Instruction::AllocStack(depth) => writeln!(target, "\tsub\trsp, {}", depth)?,
            Instruction::DeallocStack(depth) => writeln!(target, "\tadd\trsp, {}", depth)?,
            Instruction::Mov(oper0, oper1) => match (oper0, oper1) {
                (Operand::Reg(reg), Operand::Im([0, 0, 0, 0, 0, 0, 0, 0])) => {
                    writeln!(target, "\txor\t{}, {}", reg, reg)?
                }
                (oper0, oper1) => writeln!(target, "\tmov\t{}, {}", oper0, oper1)?,
            },
            Instruction::Call(name) => writeln!(target, "call\t{}", file_format.mangle(&name))?,
        }
    }
    Ok(())
}
