use crate::{Parser, types::PError};

pub fn okparser<'s, T: Clone>(v: T) -> Parser!['s, T] {
    move |src| Ok((src, v.clone()))
}

pub fn satisfy<'s>(pred: impl Fn(char) -> bool + Clone + 's) -> Parser!['s, char] {
    move |mut src| {
        match src.cur_char {
            Some(ch) if pred.clone()(ch) => {
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

pub fn optional<'s, T>(p: Parser!['s, T]) -> Parser!['s, Option<T>] {
    move |src| {
        p(src.clone())
            .map(|(new_src, v)| (new_src, Some(v)))
            .or(Ok((src, None)))
    }
}

pub fn chr<'s>(expected: char) -> Parser!['s, char] {
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

pub fn keyword<'s>(expected: &'s str) -> Parser!['_, ()] {
    move |src| {
        let mut src = src;
        for ch in expected.chars() {
            let Ok((new_src, _)) = chr(ch)(src.clone()) else {
                return Err(PError {
                    msg: format!(
                        "[keyword] Expected '{expected}', found '{}'",
                        src.cur_char.map(String::from).unwrap_or("EOF".into())
                    ),
                    ctx: src.ctx,
                });
            };
            src = new_src
        }
        Ok((src, ()))
    }
}

/// Map a function to a parser to transform the underlying type
/// ```
/// let src = make_cursor("a");
/// let p = pmap(chr('a'), |c| c.to_ascii_uppercase());
/// let (_, v) = p(src).unwrap();
/// assert_eq!(v, 'A');
/// ```
pub fn pmap<'s, A, B: Clone>(p: Parser!['s, A], f: impl Fn(A) -> B) -> Parser!['s, B] {
    move |src| {
        let (src, a) = p(src)?;
        okparser(f(a))(src)
    }
}
