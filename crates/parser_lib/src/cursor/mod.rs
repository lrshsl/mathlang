#[cfg(test)]
mod tests;

use std::str::Chars;

use crate::types::FileContext;

#[derive(Debug, Clone)]
pub struct Cursor<'s> {
    pub ctx: FileContext,
    pub src: &'s str,
    pub remainder: &'s str,
    pub chars: Chars<'s>,

    pub cur_char: Option<char>,
}

impl<'s> Cursor<'s> {
    pub fn new(src: &'s str) -> Self {
        let mut chars = src.chars();
        let first_char = chars.next();
        Self {
            ctx: FileContext::default(),
            src: src,
            remainder: src,
            chars: chars,
            cur_char: first_char,
        }
    }

    pub fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }
}

impl<'s> Iterator for Cursor<'s> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur_char = self.chars.next();
        self.ctx.col += 1;
        if let Some(ch) = self.cur_char {
            if ch == '\n' {
                self.ctx.line += 1;
                self.ctx.col = 0;
            }
            self.remainder = &self.remainder[1..];
            Some(ch)
        } else {
            self.remainder = "";
            self.cur_char = None;
            None
        }
    }
}
