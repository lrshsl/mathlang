use crate::{
    Parser,
    parser::{
        cursor::Cursor,
        parser_lib::primitives::satisfy,
        types::{PError, PResult},
    },
};

#[macro_export]
macro_rules! parse {
    ($parser:expr, $msg:expr, $src:expr) => {
        $parser($src).map_err(|mut e| {
            e.msg = format!("{}: {}", $msg, e.msg);
            e
        })
    };
}

/// Basically choice![] with pattern matching
#[macro_export]
macro_rules! pmatch {
    // single parser with default action (x => x)
    (
        $src:expr; err = $err:expr;
        $p:expr

    ) => {
        pmatch!($src; err = $err; $p, x => x)
    };


    // single parser + action
    (
        $src:expr; err = $err:expr;
        $p:expr, $pattern:pat => $action:expr

    ) => {
        match $p($src.clone()) {
            Ok((src, $pattern)) => Ok((src, $action)),
            Err(mut e) => {
                e.msg = $err.to_string();
                Err(e)
            }
        }
    };


    // multiple parser arms (fallthrough)
    (

        $src:expr; err = $err:expr;
        $p1:expr, $pat1:pat => $act1:expr;

        $( $p:expr, $($pat:pat => $act:expr)? );+
        $(;)?

    ) => {{
        let res = pmatch!($src; err = $err; $p1, $pat1 => $act1); // First parser does not need the error
        $(
            let res = match res {
                Ok(v) => Ok(v),
                Err(_) => pmatch!($src; err = $err; $p, $( $pat => $act )?),
            };
        )+
        res
    }};
}

pub fn whitespace<'s>(mut src: Cursor<'s>) -> PResult<'s, ()> {
    while let Some(ch) = src.cur_char
        && ch.is_whitespace()
    {
        src.next();
    }
    Ok((src, ()))
}

pub fn tok<'s, O>(f: Parser!['s, O]) -> Parser!['s, O] {
    move |src| {
        let (src, ()) = whitespace(src).expect("Always succeeds");
        f(src)
    }
}

pub fn digit<'s>(radix: u32) -> Parser!['s, char] {
    satisfy(move |ch| char::is_digit(ch, radix))
}

pub fn ident<'s>(mut src: Cursor<'s>) -> PResult<'s, &'s str> {
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

    let remainder = src.remainder;
    let end = src
        .position(|c| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(remainder.len() - 1);

    Ok((src, &remainder[..end + 1]))
}
