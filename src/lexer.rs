pub struct Lexer<'a> {
    pub content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self { Self { content } }

    pub fn trim_left(&mut self) {
        let mut n = 0;
        while n < self.content.len() && self.content[n].is_whitespace() {
            // self.content = &self.content[1..];
            n += 1;
        }
        self.content = &self.content[n..];
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[..n];
        self.content = &self.content[n..];
        token
    }

    pub fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
        where P: FnMut(&char) -> bool
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        loop {
            // Ignore the comments
            self.trim_left();
            if self.content.len() > 1 && self.content[0] == '/' && self.content[1] == '/' {
                self.chop_while(|&x| x != '\n');
                self.content = &self.content[1..];
            } else {
                break;
            }
        }

        if self.content.len() == 0 {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()));
        } else if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphanumeric()));
        }

        Some(self.chop(1))
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}
