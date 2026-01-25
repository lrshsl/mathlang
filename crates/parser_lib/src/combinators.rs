use crate::{
    Parser,
    cursor::Cursor,
    primitives::pmap,
    types::{BoxedParser, PError, PResult},
};

pub fn or<'s, T>(p1: Parser!['s, T], p2: Parser!['s, T]) -> Parser!['s, T] {
    move |src| match p1(src.clone()) {
        Ok(val) => Ok(val),
        Err(_) => p2(src),
    }
}

pub fn preceded<'s, T, D>(p1: Parser!['s, T], p2: Parser!['s, D]) -> Parser!['s, T] {
    move |src| {
        let (src, _) = p2(src.clone())?;
        let (src, v) = p1(src.clone())?;
        Ok((src, v))
    }
}

pub fn terminated<'s, T, D>(p1: Parser!['s, T], p2: Parser!['s, D]) -> Parser!['s, T] {
    move |src| {
        let (src, v) = p1(src)?;
        let (src, _) = p2(src)?;
        Ok((src, v))
    }
}

pub fn between<'s, T, D1, D2>(
    p: Parser!['s, T],
    d1: Parser!['s, D1],
    d2: Parser!['s, D2],
) -> Parser!['s, T] {
    move |src| {
        let (src, _) = d1(src)?;
        let (src, v) = p(src)?;
        let (src, _) = d2(src)?;
        Ok((src, v))
    }
}

pub fn choice_f<'s, T>(parsers: Vec<BoxedParser<'s, T>>) -> Parser!['s, T] {
    move |src| {
        let mut last_err = None;

        for parser in &parsers {
            match parser(src.clone()) {
                Ok(ok) => return Ok(ok),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap_or(PError {
            msg: "no matching parser".into(),
            ctx: src.ctx,
        }))
    }
}

#[macro_export]
macro_rules! choice {
    ( $( $x:expr ),+ $(,)? ) => {
        $crate::combinators::choice_f(vec![
            $(
                Box::new($x)
            ),+
        ])
    };
}

pub fn many0<'s, T>(p: Parser!['s, T]) -> Parser!['s, Vec<T>] {
    move |mut src| {
        let mut out = Vec::new();
        loop {
            match p(src.clone()) {
                Ok((next_src, v)) => {
                    // Prevent infinite loops: ensure progress
                    if next_src.remainder.len() == src.remainder.len() {
                        panic!("src not advanced");
                    }
                    src = next_src;
                    out.push(v);
                }
                Err(_) => break,
            }
        }
        Ok((src, out))
    }
}

pub fn some<'s, T>(p: Parser!['s, T]) -> Parser!['s, Vec<T>] {
    move |src| {
        // Try the first element
        let (mut src, first) = p(src.clone()).map_err(|_| PError {
            msg: "Expected at least one element".into(),
            ctx: src.ctx,
        })?;

        // Inline the rest of many0 instead of calling it
        let mut out = vec![first];
        loop {
            match p(src.clone()) {
                Ok((next_src, v)) => {
                    if next_src.remainder.len() == src.remainder.len() {
                        break;
                    }
                    src = next_src;
                    out.push(v);
                }
                Err(_) => break,
            }
        }

        Ok((src, out))
    }
}

pub fn then_append<'s, T>(ps: Parser!['s, Vec<T>], p: Parser!['s, T]) -> Parser!['s, Vec<T>] {
    move |src| {
        let (src, mut xs) = ps(src)?;
        let (src, x) = p(src)?;
        xs.push(x);
        Ok((src, xs))
    }
}

/// Parses `p` delimited by `del`. Requires will fail on empty input.
///
/// ```rust
/// # use parser_lib::combinators::delimited1;
/// # use parser_lib::primitives::chr;
/// # use parser_lib::helpers::{ident, tok};
/// # use parser_lib::cursor::Cursor;
///
/// let src = Cursor::new("a, b, c");
/// let (_, result) = delimited1(tok(ident), tok(chr(',')))(src).unwrap();
/// assert_eq!(result, vec!["a", "b", "c"])
/// ```
pub fn delimited1<'s, T, Del>(p: Parser!['s, T], del: Parser!['s, Del]) -> Parser!['s, Vec<T>] {
    move |src| then_append(many0(terminated(&p, &del)), &p)(src)
}
