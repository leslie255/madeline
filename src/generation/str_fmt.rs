#[allow(dead_code)]
pub fn asm_str_from(bytes: &Vec<u8>) -> String {
    let mut result = String::new();
    for b in bytes {
        result.push_str(format!("{b:#02X}, ").as_str());
    }
    result
}
