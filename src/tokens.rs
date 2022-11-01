#![allow(dead_code)]

use std::{iter::Peekable, rc::Rc, str::Chars};

use super::ir::DataType;

#[derive(Debug, Clone)]
pub enum Token {
    Fn,
    Extern,
    Store,
    Load,
    Call,
    Alloc,
    Ret,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Not,
    And,
    Or,
    Xor,

    Equal,
    Comma,
    Dot,
    SmallerThan,
    ParenOpen,
    ParenClose,
    RectParenOpen,
    RectParenClose,
    BraceOpen,
    BraceClose,

    NumU(u64),
    NumI(i64),
    NumF(f64),

    String(Vec<u8>),

    FnName(Rc<String>),
    RegID(u64),
    ArgID(u64),
    TypeName(DataType),

    LineBreak,
}

pub fn parse_into_tokens(source: String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars_iter = source.chars().peekable();
    loop {
        let first_ch = match chars_iter.next() {
            Some(c) => c,
            None => break,
        };
        if first_ch.is_whitespace() {
            if first_ch == '\n' {
                tokens.push(Token::LineBreak);
            }
            continue;
        }
        if first_ch.is_ascii_alphabetic() {
            let mut word = String::with_capacity(6);
            word.push(first_ch);
            while let Some(ch) = chars_iter.next_if(|c| c.is_ascii_alphanumeric()) {
                word.push(ch);
            }
            match word.as_str() {
                "fn" => tokens.push(Token::Fn),
                "extern" => tokens.push(Token::Extern),
                "store" => tokens.push(Token::Store),
                "load" => tokens.push(Token::Load),
                "call" => tokens.push(Token::Call),
                "alloc" => tokens.push(Token::Alloc),
                "ret" => tokens.push(Token::Ret),
                "u64" => tokens.push(Token::TypeName(DataType::U64)),
                "u32" => tokens.push(Token::TypeName(DataType::U32)),
                "u16" => tokens.push(Token::TypeName(DataType::U16)),
                "u8" => tokens.push(Token::TypeName(DataType::U8)),
                "usize" => tokens.push(Token::TypeName(DataType::USize)),
                "i64" => tokens.push(Token::TypeName(DataType::I64)),
                "i32" => tokens.push(Token::TypeName(DataType::I32)),
                "i16" => tokens.push(Token::TypeName(DataType::I16)),
                "i8" => tokens.push(Token::TypeName(DataType::I8)),
                "isize" => tokens.push(Token::TypeName(DataType::ISize)),
                "f64" => tokens.push(Token::TypeName(DataType::F64)),
                "f32" => tokens.push(Token::TypeName(DataType::F32)),
                "ptr" => tokens.push(Token::TypeName(DataType::Ptr)),
                _ => panic!("{word:?} is not a valid keyword, if it's an identifier, use @{word}"),
            }
            continue;
        }
        macro_rules! collect_ch {
            ($should_collect: expr) => {{
                let mut str = String::new();
                while let Some(c) = chars_iter.next_if($should_collect) {
                    str.push(c);
                }
                str
            }};
        }
        match first_ch {
            '=' => tokens.push(Token::Equal),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '(' => tokens.push(Token::ParenOpen),
            ')' => tokens.push(Token::ParenClose),
            '[' => tokens.push(Token::RectParenOpen),
            ']' => tokens.push(Token::RectParenClose),
            '{' => tokens.push(Token::BraceOpen),
            '}' => tokens.push(Token::BraceClose),
            '$' => {
                tokens.push(parse_number(collect_ch!(|c| c.is_ascii_alphanumeric()
                    || *c == '-'
                    || *c == '.')));
            }
            '%' => tokens.push(Token::RegID(
                (collect_ch!(|c| c.is_numeric())).parse().unwrap(),
            )),
            '#' => tokens.push(Token::ArgID(
                (collect_ch!(|c| c.is_numeric())).parse().unwrap(),
            )),
            '\"' => tokens.push(parse_string(&mut chars_iter)),
            '@' => tokens.push(Token::FnName(Rc::new(collect_ch!(|c| c
                .is_ascii_alphanumeric()
                || *c == '_'
                || *c == '.')))),
            '/' => while chars_iter.next_if(|c| *c != '\n').is_some() {},
            _ => panic!("cannot recognize {first_ch:?}"),
        }
    }
    tokens
}

macro_rules! hex_digit_to_num {
    ($c: expr, $t: ty) => {{
        match $c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => $c as $t - 0x30,
            'A' | 'B' | 'C' | 'D' | 'E' | 'F' => $c as $t - 0x41 + 10,
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' => $c as $t - 0x61 + 10,
            _ => panic!("Invalid digit in hexadecimal number: {:?}", $c),
        }
    }};
}

macro_rules! oct_digit_to_num {
    ($c: expr, $t: ty) => {{
        match $c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => $c as $t - 0x30,
            _ => panic!("Invalid digit in octal number: {:?}", $c),
        }
    }};
}

macro_rules! den_digit_to_num {
    ($c: expr, $t: ty) => {{
        match $c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => $c as $t - 0x30,
            _ => panic!("Invalid digit in denary number: {:?}", $c),
        }
    }};
}

fn parse_number(str: String) -> Token {
    let mut chars_iter = str.chars();
    let first_ch = chars_iter.next().expect("Empty number literal");
    let has_dot = str.chars().find(|c| *c == '.').is_some();
    if first_ch == '0' {
        if let Some(second_ch) = chars_iter.next() {
            match second_ch {
                'x' => {
                    let mut num = 0;
                    let mut weight = 1;
                    for c in str[2..].chars().rev() {
                        num += hex_digit_to_num!(c, u64) * weight;
                        let (mul_result, overflowed) = weight.overflowing_mul(16);
                        weight = if overflowed {
                            panic!("hexadecimal number exeeds 64 bit (16 hex digits)");
                        } else {
                            mul_result
                        }
                    }
                    return Token::NumU(num);
                }
                'd' => return Token::NumU(str[2..].parse().expect("Invalid denary number format")),
                'o' => {
                    let mut num = 0;
                    let mut weight = 1;
                    for c in str[2..].chars().rev() {
                        num += oct_digit_to_num!(c, u64) * weight;
                        let (add_result, overflowed) =
                            num.overflowing_add(oct_digit_to_num!(c, u64) * weight);
                        num = if overflowed {
                            panic!("Octal number literal exeeds 64 bit");
                        } else {
                            add_result
                        };
                        let (mul_result, overflowed) = weight.overflowing_mul(8);
                        weight = if overflowed {
                            panic!("Octal number literal exeeds 64 bit");
                        } else {
                            mul_result
                        }
                    }
                    return Token::NumU(num);
                }
                'b' => {
                    let mut num = 0;
                    let mut weight = 1u64;
                    for c in str[2..].chars().rev() {
                        match c {
                            '0' => (),
                            '1' => num += weight,
                            _ => panic!("Invalid digit in binary number: {c:?}"),
                        }
                        let (mul_result, overflowed) = weight.overflowing_mul(2);
                        weight = if overflowed {
                            panic!("Binary number literal exeeds 64 bit");
                        } else {
                            mul_result
                        }
                    }
                    return Token::NumU(num);
                }
                _ => (),
            }
        } else {
            return Token::NumU(0);
        }
    }
    if first_ch == '-' {
        Token::NumI(str.parse().expect("Invalid number format"))
    } else if has_dot {
        Token::NumF(str.parse().expect("Invalid number format"))
    } else {
        Token::NumU(str.parse().expect("Invalid number format"))
    }
}

fn parse_string(chars_iter: &mut Peekable<Chars>) -> Token {
    let mut bytes = Vec::<u8>::new();
    loop {
        let ch = chars_iter.next().expect("Unexpected EOF inside string");
        match ch {
            '\\' => {
                match chars_iter
                    .next()
                    .expect("Unexpected EOF inside string escape sequence")
                {
                    'x' => {
                        let digits: [char; 2] = [
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects two digits after \\x"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects two digits after \\x"),
                        ];
                        let mut num = 0;
                        num += hex_digit_to_num!(digits[0], u8) * 16;
                        num += hex_digit_to_num!(digits[1], u8);
                        bytes.push(num);
                    }
                    'o' => {
                        let digits: [char; 3] = [
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\o"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\o"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\o"),
                        ];
                        let mut num = 0;
                        num += oct_digit_to_num!(digits[0], u8) * 64;
                        num += oct_digit_to_num!(digits[1], u8) * 8;
                        num += oct_digit_to_num!(digits[2], u8);
                        bytes.push(num);
                    }
                    'd' => {
                        let digits: [char; 3] = [
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects three digits after \\d"),
                        ];
                        let mut num = 0;
                        num += den_digit_to_num!(digits[0], u8) * 100;
                        num += den_digit_to_num!(digits[1], u8) * 10;
                        num += den_digit_to_num!(digits[2], u8);
                        bytes.push(num);
                    }
                    'b' => {
                        let digits: [char; 8] = [
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                            chars_iter.next().expect("Unexpected EOF in string escape sequence, expects eight digits after \\d"),
                        ];
                        let mut num = 0;
                        let mut weight: u8 = 1;
                        for c in digits.into_iter().rev() {
                            match c {
                                '0' => (),
                                '1' => num += weight,
                                c => panic!("Invalid digit in a binary number: {c:?}"),
                            }
                            weight = weight.wrapping_mul(2);
                        }
                        bytes.push(num);
                    }
                    'n' => bytes.push(0x0A),
                    't' => bytes.push(0x09),
                    '\\' => bytes.push(0x5C),
                    '\"' => bytes.push(0x22),
                    '\'' => bytes.push(0x27),
                    _ => panic!("{ch:?} is not a valid string escape sequence"),
                }
            }
            '"' => break,
            _ => String::from(ch)
                .as_bytes()
                .into_iter()
                .for_each(|b| bytes.push(*b)),
        }
    }
    println!();
    Token::String(bytes)
}
