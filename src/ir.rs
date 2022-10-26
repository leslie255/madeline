use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Unsigned64,
    Unsigned32,
    Unsigned16,
    Unsigned8,
    Signed64,
    Signed32,
    Signed16,
    Signed8,
    Float64,
    Float32,
    Pointer,
    UnsignedSize,
    SignedSize,
    Irrelavent,
}
impl DataType {
    pub fn from_str(s: &String) -> Option<Self> {
        match s.as_str() {
            "u64" => Some(Self::Unsigned64),
            "u32" => Some(Self::Unsigned32),
            "u16" => Some(Self::Unsigned16),
            "u8" => Some(Self::Unsigned8),
            "i64" => Some(Self::Signed64),
            "i32" => Some(Self::Signed32),
            "i16" => Some(Self::Signed16),
            "i8" => Some(Self::Signed8),
            "f64" => Some(Self::Float64),
            "f32" => Some(Self::Float32),
            "ptr" => Some(Self::Pointer),
            "usize" => Some(Self::UnsignedSize),
            "isize" => Some(Self::SignedSize),
            "_" => Some(Self::Irrelavent),
            _ => None,
        }
    }
}
impl std::fmt::Display for DataType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} ",
            match &self {
                Self::Unsigned64 => "u64",
                Self::Unsigned32 => "u32",
                Self::Unsigned16 => "u16",
                Self::Unsigned8 => "u8",
                Self::Signed64 => "i64",
                Self::Signed32 => "i32",
                Self::Signed16 => "i16",
                Self::Signed8 => "i8",
                Self::Float64 => "f64",
                Self::Float32 => "f32",
                Self::Pointer => "ptr",
                Self::UnsignedSize => "usize",
                Self::SignedSize => "isize",
                Self::Irrelavent => "_",
            }
        )?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub enum OperandContent {
    Data(u64),
    Var(u64),
    SVar(Rc<String>),
    Arg(u64),
    Result,
    Fn(Rc<String>),
    Label(Rc<String>),
    SubBlock,
    RawASM(Rc<String>),
    Irrelavent,
}
impl PartialEq for OperandContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Data(x), Self::Data(y)) => x == y,
            (Self::Var(s0), Self::Var(s1)) => s0 == s1,
            (Self::Arg(x), Self::Arg(y)) => x == y,
            (Self::Result, Self::Result) => true,
            (Self::Fn(s0), Self::Fn(s1)) => s0 == s1,
            (Self::Label(s0), Self::Label(s1)) => s0 == s1,
            (Self::SubBlock, Self::SubBlock) => true,
            (Self::Irrelavent, Self::Irrelavent) => true,
            _ => false,
        }
    }
}

impl OperandContent {
    pub fn expect_data(&self) -> &u64 {
        if let Self::Data(i) = self {
            i
        } else {
            panic!("expects data")
        }
    }
    pub fn expect_var(&self) -> &u64 {
        if let Self::Var(s) = self {
            s
        } else {
            panic!("expects var")
        }
    }
    pub fn expect_arg(&self) -> &u64 {
        if let Self::Arg(i) = self {
            i
        } else {
            panic!("expects arg")
        }
    }
    pub fn expect_ret_val(&self) {
        if *self != Self::Result {
            panic!("expects ret_val")
        }
    }
    pub fn expect_fn(&self) -> &String {
        if let Self::Fn(s) = self {
            s
        } else {
            panic!("expects fn")
        }
    }
    pub fn expect_empty(&self) {
        if *self != Self::Irrelavent {
            panic!("expects _")
        }
    }
    pub fn expect_label(&self) -> &String {
        if let Self::Label(s) = self {
            s
        } else {
            panic!("expects label")
        }
    }
    pub fn expect_raw_asm(&self) -> &String {
        if let Self::RawASM(a) = self {
            a
        } else {
            panic!("expects raw asm")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Operand {
    pub dtype: DataType,
    pub content: OperandContent,
}
impl Operand {
    pub fn is_irrelavent(&self) -> bool {
        DataType::Irrelavent == self.dtype && OperandContent::Irrelavent == self.content
    }
}
impl Default for Operand {
    fn default() -> Self {
        Operand {
            dtype: DataType::Irrelavent,
            content: OperandContent::Irrelavent,
        }
    }
}
impl std::fmt::Display for Operand {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} ",
            match &self.content {
                OperandContent::Data(_) => "data",
                OperandContent::Var(_) => "var",
                OperandContent::SVar(_) => "svar",
                OperandContent::Arg(_) => "arg",
                OperandContent::Result => "result",
                OperandContent::Fn(_) => "fn",
                OperandContent::Label(_) => "label",
                OperandContent::SubBlock => "sub_block",
                OperandContent::RawASM(_) => "raw_asm",
                OperandContent::Irrelavent => "_",
            }
        )?;
        write!(formatter, "{} ", self.dtype)?;
        write!(
            formatter,
            "{}",
            match &self.content {
                OperandContent::Data(i) => format!("{i}"),
                OperandContent::Arg(i) => format!("{i}"),
                OperandContent::Var(id) => format!("{id}"),
                OperandContent::SVar(name) => format!("{name}"),
                OperandContent::Result => format!("result"),
                OperandContent::Fn(name) => format!("{name}"),
                OperandContent::Label(name) => format!("{name}"),
                OperandContent::SubBlock => format!("sub_block"),
                OperandContent::RawASM(s) => format!("{s}"),
                OperandContent::Irrelavent => format!("_"),
            }
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    DefVar,
    SetVar,
    SetArg,
    TakeAddr,
    Deref,
    CallFn,
    RetVal,
    RetVoid,
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,
    Label,
    BlockStart,
    BlockEnd,
    RawASM,
    Jmp,
    Cmp,
    Je,
    Jn,
    Jz,
    Jnz,
    Jg,
    Jl,
    Jle,
    Jge,
}
impl OperationType {
    pub fn from_str(s: &String) -> Option<Self> {
        match s.as_str() {
            "def_var" => Some(Self::DefVar),
            "set_var" => Some(Self::SetVar),
            "set_arg" => Some(Self::SetArg),
            "call_fn" => Some(Self::CallFn),
            "ret_val" => Some(Self::RetVal),
            "ret_void" => Some(Self::RetVoid),
            "add" => Some(Self::Add),
            "sub" => Some(Self::Sub),
            "mul" => Some(Self::Mul),
            "div" => Some(Self::Div),
            "inc" => Some(Self::Inc),
            "dec" => Some(Self::Dec),
            "jmp" => Some(Self::Jmp),
            "cmp" => Some(Self::Cmp),
            "j=" => Some(Self::Je),
            "j!=" => Some(Self::Jn),
            "j=0" => Some(Self::Jz),
            "j!=0" => Some(Self::Jnz),
            "j>" => Some(Self::Jg),
            "j>=" => Some(Self::Jge),
            "j<" => Some(Self::Jl),
            "j<=" => Some(Self::Jge),
            "def_label" => Some(Self::Label),
            "#block_start" => Some(Self::BlockStart),
            "#block_end" => Some(Self::BlockEnd),
            "raw_asm" => Some(Self::RawASM),
            _ => None,
        }
    }
}
impl std::fmt::Display for OperationType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                Self::DefVar => "def_var",
                Self::SetVar => "set_var",
                Self::SetArg => "set_arg",
                Self::Deref => "deref",
                Self::TakeAddr => "take_addr",
                Self::CallFn => "call_fn",
                Self::RetVal => "ret_val",
                Self::RetVoid => "ret_void",
                Self::Add => "add",
                Self::Sub => "sub",
                Self::Mul => "mul",
                Self::Div => "div",
                Self::Inc => "inc",
                Self::Dec => "dec",
                Self::Label => "label",
                Self::BlockStart => "block_start",
                Self::BlockEnd => "block_end",
                Self::RawASM => "raw_asm",
                Self::Jmp => "jmp",
                Self::Cmp => "cmp",
                Self::Je => "j=",
                Self::Jn => "j!=",
                Self::Jz => "j=0",
                Self::Jnz => "j!=0",
                Self::Jg => "j>",
                Self::Jge => "j>=",
                Self::Jl => "j<",
                Self::Jle => "j<=",
            }
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: OperationType,
    pub operand0: Operand,
    pub operand1: Operand,
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatter,
            "{}\t{}\t{}",
            self.operation, self.operand0, self.operand1
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TopLevelElement {
    FnDef(Rc<String>, Vec<Instruction>),
    // function name, instructions
    DataStr(String, String),
    // label name, content
    Extern(Rc<String>),
    // variable name, variable data type
    StaticVar(Rc<String>, DataType),
}
impl std::fmt::Display for TopLevelElement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FnDef(name, instructions) => {
                writeln!(formatter, "#fn_def {name} {{")?;
                for instruction in instructions {
                    instruction.fmt(formatter)?;
                    writeln!(formatter)?;
                }
                writeln!(formatter, "}}")?;
            }
            Self::DataStr(id, content) => {
                writeln!(formatter, "{id}\t{content}")?;
            }
            Self::Extern(label) => writeln!(formatter, "extern {label}")?,
            Self::StaticVar(name, dtype) => {
                writeln!(formatter, "#static_var {name} {dtype}")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub content: Vec<TopLevelElement>,
}
