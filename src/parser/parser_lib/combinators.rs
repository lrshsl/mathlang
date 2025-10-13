use crate::{
    Parser,
    parser::types::{BoxedParser, PError},
};

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
        $crate::parser::parser_lib::combinators::choice_f(vec![
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
