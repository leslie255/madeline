use super::tokens::*;

use std::collections::HashMap;

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
            "_" => Some(Self::Irrelavent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperandContent {
    Data(u64),
    Var(String),
    Arg(u64),
    RetVal,
    Fn(String),
    Irrelavent,
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub dtype: DataType,
    pub content: OperandContent,
}
impl Operand {
    fn is_irrelavent(&self) -> bool {
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

#[derive(Debug, Clone, Copy)]
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
    FnDef(String, HashMap<String, DataType>, Vec<Instruction>),
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
                    let mut vars: HashMap<String, DataType> = HashMap::new();
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
                        vars.insert(var_name, dtype);
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
            _ => panic!("cannot recognize operand type"),
        },
    }
}

pub fn parse_instr(tokens: &mut TokenStream, current: String) -> Instruction {
    let id = current;
    let operation =
        OperationType::from_str(&id).unwrap_or_else(|| panic!("cannot recognize op `{}`", id));
    let operand0 = parse_operand(tokens);
    let operand1 = parse_operand(tokens);
    if !operand1.is_irrelavent() {
        if tokens.next() != Some(String::from(";")) {
            panic!("expected `;`");
        }
    }

    Instruction {
        operation,
        operand0,
        operand1,
    }
}
