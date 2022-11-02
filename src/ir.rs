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

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Var(DataType, u64),
    Arg(DataType, u64),
    Reg(DataType, u64),
    UInt(DataType, u64),
    Int(DataType, i64),
    Float(DataType, f64),
    String(Vec<u8>),

    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),

    Load {
        id: u64,
        dtype: DataType,
    },

    Alloc(DataType),
    DefReg {
        id: u64,
        rhs: Box<Self>,
    },

    Store {
        id: u64,
        rhs: Box<Self>,
    },
    Ret(Option<Box<Self>>),

    Call {
        ret_type: Option<DataType>,
        fn_name: Rc<String>,
        args: Vec<Self>,
    },

    Label(Rc<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    Extern(Rc<String>),
    Fn {
        name: Rc<String>,
        args: Vec<DataType>,
        body: Vec<Instruction>,
    },
}
