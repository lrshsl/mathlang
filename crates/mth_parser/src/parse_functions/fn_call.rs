use parser_lib::{choice, types::Parser};

use super::*;

pub fn parse_fn_call(src: Cursor) -> PResult<FunctionCall> {
    let (src, name) = parse!(tok(ident), "Couldn't parse function name", src)?;

    // Parse comma-separated arguments
    let parse_args = delimited0(tok(expr), tok(chr(',')));
    let (src, args) = parse!(
        between(parse_args, tok(chr('(')), tok(chr(')'))),
        "Couldn't parse function arguments",
        src
    )?;

    Ok((
        src,
        FunctionCall {
            name,
            args,
            is_negated: false,
        },
    ))
}

pub fn parse_op<'s>() -> impl Parser<'s, &'s str> {
    preceded(
        whitespace,
        choice!(
            keyword("=="),
            keyword("!="),
            keyword("<="),
            keyword(">="),
            keyword("<"),
            keyword(">"),
            keyword("+"),
            keyword("-"),
            keyword("*"),
            keyword("/"),
            keyword("^"),
            keyword("and"),
            keyword("or")
        ),
    )
}
