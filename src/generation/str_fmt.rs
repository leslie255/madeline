pub fn asm_str_from(string: String) -> String {
    let mut result = String::new();
    for ch in string.bytes() {
        result.push_str(format!("{ch:#04X}, ").as_str());
    }
    result
}
