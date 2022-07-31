#![allow(unused)]
use std::collections::HashMap;

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

pub enum OperandContent {
    Data(u64),
    Var(String),
    Arg(u64),
    RetVal,
    Fn(String),
}

pub struct Operand {
    pub length: DataType,
    pub content: OperandContent,
}

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

pub struct Instruction {
    pub operation: OperationType,
    pub operand0: Operand,
    pub operand1: Operand,
}

pub enum TopLevelElement {
    FnDef(String, HashMap<String, DataType>, Vec<Instruction>),
    // function name, variables, instructions
}

