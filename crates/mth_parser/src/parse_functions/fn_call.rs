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

pub fn parse_s_expr(src: Cursor) -> PResult<FunctionCall> {
    let ident_or_symbol = or(parse_op(), ident);
    let (src, name) = parse!(tok(ident_or_symbol), "Couldn't parse s_expr name", src)?;
    let (src, args) = parse!(some(primary), "Couldn't parse argument", src)?;

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
            keyword("^")
        ),
    )
}
