use super::tokens::*;

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
    Irrelavent,
}
impl DataType {
    fn from_str(s: &String) -> Option<Self> {
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
            "_" => Some(Self::Irrelavent),
            _ => None,
        }
    }
    pub fn size(&self) -> u64 {
        match self {
            Self::Unsigned64 => 8,
            Self::Unsigned32 => 4,
            Self::Unsigned16 => 2,
            Self::Unsigned8 => 1,
            Self::Signed64 => 8,
            Self::Signed32 => 4,
            Self::Signed16 => 2,
            Self::Signed8 => 1,
            Self::Float64 => 8,
            Self::Float32 => 4,
            Self::Irrelavent => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperandContent {
    Data(u64),
    Var(String),
    Arg(u64),
    RetVal,
    Fn(String),
    Label(String),
    SubBlock(Vec<Instruction>),
    RawASM(String),
    Irrelavent,
}
impl PartialEq for OperandContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Data(x), Self::Data(y)) => x == y,
            (Self::Var(s0), Self::Var(s1)) => s0 == s1,
            (Self::Arg(x), Self::Arg(y)) => x == y,
            (Self::RetVal, Self::RetVal) => true,
            (Self::Fn(s0), Self::Fn(s1)) => s0 == s1,
            (Self::Label(s0), Self::Label(s1)) => s0 == s1,
            (Self::SubBlock(..), Self::SubBlock(..)) => true,
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
    pub fn expect_var(&self) -> &String {
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
        if *self != Self::RetVal {
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
            panic!("expects `;`")
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    SetVar,
    SetArg,
    CallFn,
    Ret,
    Add,
    Sub,
    Mul,
    Div,
    Inc,
    Dec,
    Label,
    BlockStart,
    RawASM,
    Jmp,
    Cmp,
    Je,
    Jn,
    Jz,
    Jnz,
    Jg,
    Jl,
    Jnge,
    Jnle,
}
impl OperationType {
    fn from_str(s: &String) -> Option<Self> {
        match s.as_str() {
            "set_var" => Some(OperationType::SetVar),
            "set_arg" => Some(OperationType::SetArg),
            "call_fn" => Some(OperationType::CallFn),
            "ret" => Some(OperationType::Ret),
            "add" => Some(OperationType::Add),
            "sub" => Some(OperationType::Sub),
            "mul" => Some(OperationType::Mul),
            "div" => Some(OperationType::Div),
            "inc" => Some(OperationType::Inc),
            "dec" => Some(OperationType::Dec),
            "jmp" => Some(OperationType::Jmp),
            "cmp" => Some(OperationType::Cmp),
            "j=" => Some(OperationType::Je),
            "j!=" => Some(OperationType::Jn),
            "j=0" => Some(OperationType::Jz),
            "j!=0" => Some(OperationType::Jnz),
            "j>" => Some(OperationType::Jg),
            "j>=" => Some(OperationType::Jnle),
            "j<" => Some(OperationType::Jl),
            "j<=" => Some(OperationType::Jnge),
            "#label" => Some(OperationType::Label),
            "#block_start" => Some(OperationType::BlockStart),
            "raw_asm" => Some(OperationType::RawASM),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub operation: OperationType,
    pub operand0: Operand,
    pub operand1: Operand,
}

#[derive(Debug, Clone)]
pub enum TopLevelElement {
    FnDef(String, Vec<(String, DataType)>, Vec<Instruction>),
    // function name, variables, instructions
}

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub content: Vec<TopLevelElement>,
}
impl Program {
    pub fn parse_from(mut token_stream: TokenStream) -> Self {
        let mut program = Program::default();
        loop {
            let mut token = match token_stream.next() {
                Some(t) => t,
                None => break,
            };
            match token.as_str() {
                "#def_fn" => {
                    let fn_name = token_stream.expected_next();
                    let mut vars: Vec<(String, DataType)> = Vec::new();
                    // parse variables
                    loop {
                        token = token_stream.expected_next();
                        if token == "{" {
                            break;
                        }
                        let var_name = token;
                        token = token_stream.expected_next();
                        let dtype = DataType::from_str(&token)
                            .unwrap_or_else(|| panic!("{} isnot a data type", token));
                        vars.push((var_name, dtype));
                    }
                    let mut body: Vec<Instruction> = Vec::new();
                    // parse body
                    loop {
                        token = token_stream.expected_next();
                        if token == "}" {
                            break;
                        }
                        body.push(parse_instr(&mut token_stream, token));
                    }
                    program
                        .content
                        .push(TopLevelElement::FnDef(fn_name, vars, body));
                }
                _ => panic!("cannot recognize {:?}", token),
            }
        }
        program
    }
}

pub fn parse_operand(tokens: &mut TokenStream) -> Operand {
    let optype = tokens.expected_next();
    if optype == ";" {
        return Operand::default();
    }
    let dtype = DataType::from_str(&tokens.expected_next()).expect("cannot recognize data type");
    let content = tokens.expected_next();

    Operand {
        dtype,
        content: match optype.as_str() {
            "data" => OperandContent::Data(content.parse().expect("not an integar")),
            "var" => OperandContent::Var(content),
            "arg" => OperandContent::Arg(content.parse().expect("not an integar")),
            "ret_val" => OperandContent::RetVal,
            "fn" => OperandContent::Fn(content),
            "label" => OperandContent::Label(content),
            "block" => todo!("block operand has not been implemented yet"),
            _ => panic!("{} is not a valid operand", optype),
        },
    }
}

pub fn parse_instr(token_stream: &mut TokenStream, current: String) -> Instruction {
    let id = current;
    let opcode =
        OperationType::from_str(&id).unwrap_or_else(|| panic!("cannot recognize op `{}`", id));

    if opcode != OperationType::RawASM {
        let operand0 = parse_operand(token_stream);
        let operand1 = parse_operand(token_stream);
        if !operand1.is_irrelavent() {
            if token_stream.next() != Some(String::from(";")) {
                panic!("expected `;`");
            }
        }
        Instruction {
            operation: opcode,
            operand0,
            operand1,
        }
    } else {
        let mut token = token_stream.expected_next();
        let mut asm = String::new();
        while token != ";" {
            asm.push_str(token.as_str());
            asm.push(' ');
            token = token_stream.expected_next();
        }
        Instruction {
            operation: opcode,
            operand0: Operand {
                dtype: DataType::Irrelavent,
                content: OperandContent::RawASM(asm),
            },
            operand1: Operand::default(),
        }
    }
}
