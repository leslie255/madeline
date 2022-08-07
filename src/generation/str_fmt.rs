pub fn asm_str_from(string: String) -> String {
    let mut result = String::new();
    for ch in string.bytes() {
        result.push_str(format!("{:#04X}, ", ch).as_str());
    }
    result
}

