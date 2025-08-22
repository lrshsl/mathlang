use std::str::Chars;

use crate::graph::ops::Instruction;

#[derive(Clone)]
pub struct Cursor<'s> {
    ctx: Context,
    remainder: &'s str,
    chars: Chars<'s>,

    cur_char: Option<char>,
}

impl<'s> Cursor<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            ctx: Context::default(),
            remainder: src,
            chars: src.chars(),
            cur_char: None,
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


#[derive(Debug, Clone)]
pub struct Context {
    pub filename: Option<String>,
    pub line: usize,
    pub col: usize,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Context {
            filename,
            line,
            col,
        } = self;
        if let Some(fname) = filename {
            write!(f, "{fname}:{line}:{col}")
        } else {
            write!(f, "{line}:{col}")
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            filename: None,
            line: 1,
            col: 0,
        }
    }
}

#[derive(Debug)]
pub struct PError {
    pub msg: String,
    pub ctx: Context,
}

impl std::fmt::Display for PError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PError { msg, ctx } = self;
        write!(f, "Error: {msg}\n   {ctx}")
    }
}

pub type PResult<'s, O> = Result<(Cursor<'s>, O), PError>;

macro_rules! Parser {
    ($lt:lifetime, $out:ty) => {
        impl Fn(Cursor<$lt>) -> PResult<$out>
    };
}

fn whitespace<'s>(mut src: Cursor<'s>) -> PResult<'s, ()> {
    while let Some(ch) = src.cur_char && ch.is_whitespace() {
        src.next();
    }
    Ok((src, ()))
}


fn tok<'s, O>(f: Parser!['s, O]) -> Parser!['s, O] {
    move |src| {
        let (src, ()) = whitespace(src).expect("Always succeeds");
        f(src)
    }
}

fn ident<'s>(mut src: Cursor<'s>) -> PResult<'s, &'s str> {
    if let Some(ch) = src.cur_char
        && ch.is_alphabetic()
    {
        let remainder = src.as_str();
        let end = src
            .position(|c| !(c.is_alphanumeric() || c == '_'))
            .unwrap_or(src.remainder.len());
        Ok((src, &remainder[..end]))
    } else {
        Err(PError {
            msg: "Ident has to start with an alphabetic character".to_string(),
            ctx: src.ctx.clone(),
        })
    }
}

pub fn parse_fn<'s>(src: Cursor<'s>) -> PResult<'s, (&'s str, Vec<Instruction>)> {
    let (src, fn_name) = tok(ident)(src).map_err(|mut e| {
        e.msg = "Expected and ident as start of a function definition".to_string();
        e
    })?;
    Err(PError {
        msg: format!("Ident {fn_name} ends:"),
        ctx: src.ctx.clone(),
    })
    // Ok((src, (fn_name, vec![inst!(OP_X_POLY, -1., 2.)])))
}
