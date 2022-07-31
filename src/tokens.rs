use std::iter::Iterator;

pub struct TokenStream<'a> {
    pub source: &'a String,
    pub iter: Box<(dyn Iterator<Item = char> + 'a)>,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a String) -> Self {
        TokenStream {
            source,
            iter: Box::new(source.chars().into_iter()),
        }
    }
    pub fn next(&mut self) -> Option<String> {
        let mut token = String::new();
        while token.is_empty() {
            loop {
                let ch = match self.iter.next() {
                    Some(c) => c,
                    None => {
                        if token.is_empty() {
                            return None;
                        } else {
                            break;
                        }
                    }
                };
                if ch == '/' {
                    while self.iter.next() != Some('\n') {}
                    break;
                }
                if ch.is_whitespace() {
                    break;
                }
                token.push(ch);
            }
        }
        Some(token)
    }
    pub fn expected_next(&mut self) -> String {
        self.next().expect("unexpected EOF")
    }
}
