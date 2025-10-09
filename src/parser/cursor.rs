use std::str::Chars;

use crate::parser::types::FileContext;

#[derive(Clone)]
pub struct Cursor<'s> {
    pub ctx: FileContext,
    pub src: &'s str,
    pub remainder: &'s str,
    pub chars: Chars<'s>,

    pub cur_char: Option<char>,
}

impl<'s> Cursor<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            ctx: FileContext::default(),
            src: src,
            remainder: src,
            chars: src.chars(),
            cur_char: src.chars().next(),
        }
    }
}

impl<'s> Iterator for Cursor<'s> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_char = self.chars.next();
        if let Some(ch) = self.cur_char {
            self.ctx.col += 1;
            if ch == '\n' {
                self.ctx.line += 1;
                self.ctx.col = 1;
            }
            self.remainder = &self.remainder[1..];
            Some(ch)
        } else {
            None
        }
    }
}
