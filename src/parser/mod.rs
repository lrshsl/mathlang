pub mod ast;
pub mod cursor;
pub mod types;

use std::collections::HashMap;

use ast::Expr;
use types::{PError, PResult};

use crate::{
    Parser,
    graph::ops::{Instruction, OP_CONST, OP_MUL, OP_X, OP_X_POLY},
    inst,
    parser::cursor::Cursor,
};

fn okparser<'s, T>(v: T) -> Parser!['s, T] {
    move |src| Ok((src, v))
}

fn chr<'s>(expected: char) -> Parser!['s, char] {
    move |mut src| match src.cur_char {
        Some(ch) if ch == expected => {
            src.next();
            Ok((src, expected))
        }
        Some(other) => Err(PError {
            msg: format!("[chr] Expected '{expected}', found '{other}'"),
            ctx: src.ctx,
        }),
        None => Err(PError {
            msg: format!("[chr] Unexpected EOF, expected '{expected}'"),
            ctx: src.ctx,
        }),
    }
}

fn whitespace<'s>(mut src: Cursor<'s>) -> PResult<'s, ()> {
    while let Some(ch) = src.cur_char
        && ch.is_whitespace()
    {
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

fn pmap<'s, A, B>(p: Parser!['s, A], f: impl FnOnce(A) -> B) -> Parser!['s, B] {
    move |src| {
        let (src, a) = p(src)?;
        okparser(f(a))(src)
    }
}

fn nr<'s>(src: Cursor<'s>) -> PResult<'s, f32> {
    let Some(ch) = src.cur_char else {
        return Err(PError {
            msg: "[nr] Unexpected EOF, expecting digit [0-9]".to_string(),
            ctx: src.ctx,
        });
    };
    if !ch.is_numeric() {
        return Err(PError {
            msg: "[nr] Number must start with a digit [0-9]".to_string(),
            ctx: src.ctx,
        });
    }

    let slice = src
        .clone()
        .take_while(|&ch| char::is_numeric(ch) || ch == '.');
    let combined = [ch].into_iter().chain(slice).collect::<String>();

    let Ok(nr) = combined.parse::<f32>() else {
        return Err(PError {
            msg: format!("[nr] Could not parse number: {}", combined),
            ctx: src.ctx,
        });
    };

    Ok((src, nr))
}

fn ident<'s>(mut src: Cursor<'s>) -> PResult<'s, &'s str> {
    let Some(ch) = src.cur_char else {
        return Err(PError {
            msg: "Unexpected EOF, expecting ident".to_string(),
            ctx: src.ctx,
        });
    };
    if !ch.is_alphabetic() {
        return Err(PError {
            msg: "Ident has to start with an alphabetic character".to_string(),
            ctx: src.ctx,
        });
    }

    let remainder = src.as_str();
    let end = src
        .position(|c| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(src.remainder.len());

    Ok((src, &remainder[..end]))
}

fn expr<'s>() -> Parser!['s, Expr] {
    move |src| {
        if let Ok((src, _)) = tok(chr('-'))(src.clone()) {
            let (src, inner) = expr()(src)?;
            return Ok((src, Expr::Minus(Box::new(inner))));
        }

        if let Ok((src, v)) = tok(nr)(src.clone()) {
            return Ok((src, Expr::Nr(v)));
        }

        if let Ok((src, v)) = tok(ident)(src.clone()) {
            return Ok((src, Expr::Ref(v)));
        }

        Err(PError {
            msg: "[expr] No parser succeeded".to_string(),
            ctx: src.ctx,
        })
    }
}

macro_rules! parse {
    ($parser:expr, $msg:expr, $src:expr) => {
        $parser($src).map_err(|mut e| {
            e.msg = format!("{}: {}", $msg, e.msg);
            e
        })
    };
}

pub fn parse_fn<'s>(src: Cursor<'s>) -> PResult<'s, (&'s str, Vec<Instruction>)> {
    let (src, fn_name) = parse!(
        ident,
        "[Fn] Expected an ident as start of a function definition",
        src
    )?;
    let (src, _) = parse!(tok(chr('(')), "[Fn] Expected '('", src)?;
    let (src, param) = parse!(
        tok(ident),
        "[Fn] Expected ident as parameter name after '('",
        src
    )?;
    let (src, _) = parse!(tok(chr(')')), "[Fn] Expected ')'", src)?;
    let (src, _) = parse!(tok(chr('=')), "[Fn] Expected '=' after ident", src)?;
    let (src, expr) = parse!(expr(), "[Fn] Expected expression", src)?;

    Ok((src, (fn_name, expr.compile())))
}
