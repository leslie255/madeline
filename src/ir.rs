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
impl DataType {
    /// Size of the data type in bytes (8, 4, 2, 1)
    /// If it's Ptr, USize or ISize, return `word_size`
    pub fn size(self, word_size: u8) -> u8 {
        match self {
            DataType::U64 | DataType::I64 | DataType::F64 => 8,
            DataType::U32 | DataType::I32 | DataType::F32 => 4,
            DataType::U16 | DataType::I16 => 2,
            DataType::U8 | DataType::I8 => 1,
            DataType::USize | DataType::ISize | DataType::Ptr => word_size,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
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

impl Instruction {
    /// Returns `true` if the instruction is [`DefReg`].
    ///
    /// [`DefReg`]: Instruction::DefReg
    #[must_use]
    pub fn is_def_reg(&self) -> bool {
        matches!(self, Self::DefReg { .. })
    }
    #[must_use]
    pub fn as_def_reg_id(&self) -> Option<u64> {
        if let Self::DefReg { id, .. } = self {
            Some(*id)
        } else {
            None
        }
    }
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
