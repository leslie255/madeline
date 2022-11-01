use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    U64,
    U32,
    U16,
    U8,
    USize,
    I64,
    I32,
    I16,
    I8,
    ISize,
    F64,
    F32,
    Ptr,
}

pub enum Instruction {
    Var(DataType, u64),
    Arg(DataType, u64),
    Reg(DataType, u64),
    UInt(DataType, u64),
    Int(DataType, i64),
    Float(DataType, f64),

    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),

    Load(Box<Self>),

    AllocVar { id: u64, dtype: DataType },
    DefReg { id: u64, dtype: DataType },

    Store { id: u64, rhs: Box<Self> },
    Ret(Box<Self>),

    Label(Rc<String>),
}

pub enum TopLevel {
    Extern(Rc<String>),
    Fn {
        name: Rc<String>,
        args: Vec<(u64, DataType)>,
        body: Vec<Instruction>,
    },
}
