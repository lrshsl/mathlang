use std::str::Chars;

use crate::parser::types::Context;

#[derive(Clone)]
pub struct Cursor<'s> {
    pub ctx: Context,
    pub remainder: &'s str,
    pub chars: Chars<'s>,

    pub cur_char: Option<char>,
}

impl<'s> Cursor<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            ctx: Context::default(),
            remainder: src,
            chars: src.chars(),
            cur_char: src.chars().next(),
        }
    }

    pub fn as_str(&self) -> &'s str {
        self.remainder
    }
}

impl<'s> Iterator for Cursor<'s> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.remainder = self.chars.as_str();
        self.cur_char = self.chars.next();
        if let Some(ch) = self.cur_char {
            self.ctx.col += 1;
            if ch == '\n' {
                self.ctx.line += 1;
                self.ctx.col = 1;
            }
            Some(ch)
        } else {
            None
        }
    }
}
