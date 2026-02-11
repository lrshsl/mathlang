use crate::{
    cursor::Cursor,
    types::{PError, Parser},
};

pub fn okparser<'s, T: Clone>(v: T) -> impl Parser<'s, T> {
    move |src| Ok((src, v.clone()))
}

pub fn satisfy<'s>(pred: impl Fn(char) -> bool) -> impl Parser<'s, char> {
    move |mut src| {
        match src.cur_char {
            Some(ch) if pred(ch) => {
                src.next(); // advances in place
                Ok((src, ch))
            }
            Some(_) => Err(PError {
                msg: "[satisfy] Predicate failed".to_string(),
                ctx: src.ctx,
            }),
            None => Err(PError {
                msg: "[satisfy] Unexpected EOF".to_string(),
                ctx: src.ctx,
            }),
        }
    }
}

pub fn optional<'s, T>(p: impl Parser<'s, T>) -> impl Parser<'s, Option<T>> {
    move |src| {
        p(src.clone())
            .map(|(new_src, v)| (new_src, Some(v)))
            .or(Ok((src, None)))
    }
}

pub fn chr<'s>(expected: char) -> impl Parser<'s, char> {
    move |mut src: Cursor<'s>| match src.cur_char {
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

pub fn chr_take_while<'s>(pred: impl Fn(&char) -> bool) -> impl Parser<'s, &'s str> {
    move |mut src| {
        let len = src
            .remainder
            .chars()
            .position(|ref ch| !pred(ch))
            .unwrap_or_else(|| src.remainder.len());
        let slice = &src.remainder[..len];
        src.advance(len);
        Ok((src, slice))
    }
}

pub fn keyword<'s, 'exp>(expected: &'exp str) -> impl Parser<'s, &'s str> + 'exp {
    move |mut src: Cursor<'s>| {
        if src.remainder.len() < expected.len() {
            return Err(PError {
                msg: format!("[keyword] Unexpected EOF. Expected: '{expected}'"),
                ctx: src.ctx,
            });
        }

        let slice = &src.remainder[..expected.len()];

        if slice != expected {
            return Err(PError {
                msg: format!("[keyword] Expected '{expected}', found '{slice}'"),
                ctx: src.ctx,
            });
        }

        src.advance(expected.len());
        Ok((src, slice))
    }
}

/// Map a function to a parser to transform the underlying type
pub fn pmap<'s, A, B: Clone>(p: impl Parser<'s, A>, f: impl Fn(A) -> B) -> impl Parser<'s, B> {
    move |src| {
        let (src, a) = p(src)?;
        okparser(f(a))(src)
    }
}
