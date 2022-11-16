use std::fmt::Display;

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