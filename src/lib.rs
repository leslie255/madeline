pub mod fileformat;
pub mod generation;
pub mod ir;
mod tokens;

#[test]
fn parse_tokens() {
    use tokens::*;
    let src = r#"
        "\"A\n\x42"
        fn @main(u32, ptr) {
            %0 = #0
            %1 = #1
            ret i32 $0
        }
        "#
    .to_string();
    let tokens = parse_into_tokens(src);
    println!("{tokens:?}");
}
