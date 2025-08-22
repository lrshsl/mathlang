use std::str::Chars;

use crate::{
    graph::ops::{Instruction, OP_X_POLY},
    inst,
};

pub struct Src<'s> {
    text: &'s str,
    remainder: Chars<'s>,
    remainder_len: usize,
    ctx: Context,
}

impl<'s> Src<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            text,
            remainder: text.chars(),
            remainder_len: text.len(),
            ctx: Context::default(),
        }
    }
}

#[derive(Clone)]
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
            col: 1,
        }
    }
}

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

pub type PResult<'s, T> = Result<(&'s Src<'s>, T), PError>;

fn ident<'s>(src: &'s mut Src<'s>) -> PResult<'s, &'s str> {
    if let Some(ch) = src.remainder.next()
        && ch.is_alphabetic()
    {
        let end = src
            .remainder
            .clone()
            .position(|c| !c.is_alphanumeric() || c == '_')
            .unwrap_or(src.remainder_len);
        src.ctx.col += end;
        Ok((src, &src.remainder.as_str()[..end]))
    } else {
        Err(PError {
            msg: "Ident has to start with an alphabetic character".to_string(),
            ctx: src.ctx.clone(),
        })
    }
}

pub fn parse_fn<'s>(src: &'s mut Src<'s>) -> PResult<'s, (&'s str, Vec<Instruction>)> {
    let (src, fn_name) = ident(src).map_err(|mut e| {
        e.msg = "Expected and ident as start of a function definition".to_string();
        e
    })?;
    Ok((src, (fn_name, vec![inst!(OP_X_POLY, -1., 2.)])))
}
