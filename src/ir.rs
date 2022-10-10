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
impl std::fmt::Display for DataType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} ",
            match &self {
                DataType::Unsigned64 => "u64",
                DataType::Unsigned32 => "u32",
                DataType::Unsigned16 => "u16",
                DataType::Unsigned8 => "u8",
                DataType::Signed64 => "i64",
                DataType::Signed32 => "i32",
                DataType::Signed16 => "i16",
                DataType::Signed8 => "i8",
                DataType::Float64 => "f64",
                DataType::Float32 => "f32",
                DataType::Irrelavent => "_",
            }
        )?;
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub enum OperandContent {
    Data(u64),
    Var(String),
    SVar(String),
    Arg(u64),
    Result,
    Fn(String),
    Label(String),
    SubBlock(Vec<(String, DataType)>),
    RawASM(String),
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
                OperandContent::SubBlock(_) => "sub_block",
                OperandContent::RawASM(_) => "raw_asm",
                OperandContent::Irrelavent => return Ok(()),
            }
        )?;
        write!(formatter, "{} ", self.dtype)?;
        write!(
            formatter,
            "{}",
            match &self.content {
                OperandContent::Data(i) => format!("{i}"),
                OperandContent::Arg(i) => format!("{i}"),
                OperandContent::Var(name) => format!("{name}"),
                OperandContent::SVar(name) => format!("{name}"),
                OperandContent::Result => format!("result"),
                OperandContent::Fn(name) => format!("{name}"),
                OperandContent::Label(name) => format!("{name}"),
                OperandContent::SubBlock(_) => format!("sub_block"),
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
    Jnge,
    Jnle,
}
impl OperationType {
    fn from_str(s: &String) -> Option<Self> {
        match s.as_str() {
            "def_var" => Some(OperationType::DefVar),
            "set_var" => Some(OperationType::SetVar),
            "set_arg" => Some(OperationType::SetArg),
            "call_fn" => Some(OperationType::CallFn),
            "ret_val" => Some(OperationType::RetVal),
            "ret_void" => Some(OperationType::RetVoid),
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
            "def_label" => Some(OperationType::Label),
            "#block_start" => Some(OperationType::BlockStart),
            "#block_end" => Some(OperationType::BlockEnd),
            "raw_asm" => Some(OperationType::RawASM),
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
                OperationType::DefVar => "def_var",
                OperationType::SetVar => "set_var",
                OperationType::SetArg => "set_arg",
                OperationType::CallFn => "call_fn",
                OperationType::RetVal => "ret_val",
                OperationType::RetVoid => "ret_void",
                OperationType::Add => "add",
                OperationType::Sub => "sub",
                OperationType::Mul => "mul",
                OperationType::Div => "div",
                OperationType::Inc => "inc",
                OperationType::Dec => "dec",
                OperationType::Label => "label",
                OperationType::BlockStart => "block_start",
                OperationType::BlockEnd => "block_end",
                OperationType::RawASM => "raw_asm",
                OperationType::Jmp => "jmp",
                OperationType::Cmp => "cmp",
                OperationType::Je => "j=",
                OperationType::Jn => "j!=",
                OperationType::Jz => "j=0",
                OperationType::Jnz => "j!=0",
                OperationType::Jg => "j>",
                OperationType::Jnle => "j>=",
                OperationType::Jl => "j<",
                OperationType::Jnge => "j<=",
            }
        )?;
        return Ok(());
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
    FnDef(String, Vec<Instruction>),
    // function name, instructions
    DataStr(String, String),
    // label name, content
    Extern(String),
    // variable name, variable data type
    StaticVar(String, DataType),
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

                    if token_stream.expected_next() != "{" {
                        panic!("expects `{{` after #def_fn {}", fn_name);
                    }

                    // parse body
                    let mut body: Vec<Instruction> = Vec::new();
                    loop {
                        token = token_stream.expected_next();
                        if token == "}" {
                            break;
                        }
                        body.push(parse_instr(&mut token_stream, token));
                    }
                    program
                        .content
                        .push(TopLevelElement::FnDef(fn_name, body));
                }
                "#data_str" => {
                    let label_name = token_stream.expected_next();
                    let mut string = String::from(token_stream.next_non_whitespace_ch().expect("Unexpected EOF"));
                    let mut is_in_escape = false;
                    while let Some(ch) = token_stream.next_ch_until('\n') {
                        if !is_in_escape {
                            if ch == '\\' {
                                is_in_escape = true;
                            } else {
                                string.push(ch);
                            }
                        } else {
                            is_in_escape = false;
                            match ch {
                                'n' => string.push('\n'),
                                '0' => string.push('\0'),
                                '\\' => string.push('\\'),
                                _ => panic!("unsupported escape character '{}'", ch),
                            }
                        }
                    }
                    program.content.push(TopLevelElement::DataStr(label_name, string));
                }
                "#extern" => {
                    let label_name = token_stream.expected_next();
                    program.content.push(TopLevelElement::Extern(label_name));
                }
                "#static_var" => {
                    let var_name = token_stream.expected_next();
                    let var_type = DataType::from_str(&token_stream.expected_next()).expect("cannot recognize data type");
                    program.content.push(TopLevelElement::StaticVar(var_name, var_type));
                }
                _ => panic!("cannot recognize {:?}, it is either not an instruction or not allowed at top level", token),
            }
        }
        program
    }

    // print the IR code for debug
    pub fn print_code(&self) {
        for toplevel_element in &self.content {
            match toplevel_element {
                TopLevelElement::FnDef(fn_name, instructions) => {
                    println!("#fn_def {fn_name} {{");
                    for instr in instructions {
                        println!(
                            "\t{}\t{}\t{}",
                            instr.operation, instr.operand0, instr.operand1
                        );
                    }
                    println!("}}");
                }
                TopLevelElement::DataStr(name, str) => {
                    println!("#data_str {name}\t{str:?}");
                }
                TopLevelElement::Extern(name) => {
                    println!("#extern {name}");
                }
                TopLevelElement::StaticVar(name, dtype) => {
                    println!("#static_var {name} {dtype}");
                }
            }
        }
    }
}

pub fn parse_operand(tokens: &mut TokenStream) -> Operand {
    let opcode_str = tokens.expected_next();
    if opcode_str == "_" {
        return Operand::default();
    }
    let dtype = DataType::from_str(&tokens.expected_next()).expect("cannot recognize data type");
    let content = tokens.expected_next();
    Operand {
        dtype,
        content: match opcode_str.as_str() {
            "data" => OperandContent::Data(content.parse().expect("not an integar")),
            "var" => OperandContent::Var(content),
            "svar" => OperandContent::SVar(content),
            "arg" => OperandContent::Arg(content.parse().expect("not an integar")),
            "result" => OperandContent::Result,
            "fn" => OperandContent::Fn(content),
            "label" => OperandContent::Label(content),
            "block" => todo!("block operand has not been implemented yet"),
            _ => panic!("{} is not a valid operand", opcode_str),
        },
    }
}

pub fn parse_instr(token_stream: &mut TokenStream, current: String) -> Instruction {
    let id = current;
    let opcode =
        OperationType::from_str(&id).unwrap_or_else(|| panic!("cannot recognize op `{}`", id));
    match opcode {
        OperationType::RawASM => {
            let mut asm = String::from(
                token_stream
                    .next_non_whitespace_ch()
                    .expect("Unexpected EOF"),
            );
            while let Some(ch) = token_stream.next_ch_until('\n') {
                asm.push(ch);
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
        OperationType::BlockStart => Instruction {
            operation: opcode,
            operand0: Operand {
                dtype: DataType::Irrelavent,
                content: OperandContent::SubBlock(Vec::new()),
            },
            operand1: Operand::default(),
        },
        _ => {
            let operand0 = parse_operand(token_stream);
            let operand1 = parse_operand(token_stream);
            Instruction {
                operation: opcode,
                operand0,
                operand1,
            }
        }
    }
}
