use std::iter::Iterator;

pub struct TokenStream<'a> {
    iter: Box<(dyn Iterator<Item = char> + 'a)>,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a String) -> Self {
        TokenStream {
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
    // give the next non whitespace character
    // returns None if EOF
    pub fn next_non_whitespace_ch(&mut self) -> Option<char> {
        let mut ch = self.iter.next()?;
        while ch.is_whitespace() {
            ch = self.iter.next()?;
        }
        Some(ch)
    }
    // give only one character until reaching a character
    // returns None if reaches `end` or EOF
    pub fn next_ch_until(&mut self, end: char) -> Option<char> {
        let ch = self.iter.next()?;
        if ch == end {
            None
        } else {
            Some(ch)
        }
    }
}
