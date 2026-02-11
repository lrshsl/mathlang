use parser_lib::{choice, types::Parser};

use super::*;

pub fn parse_fn_call(src: Cursor) -> PResult<FunctionCall> {
    let parse_args = delimited1(tok(expr), tok(chr(',')));

    let (src, name) = parse!(tok(ident), "Couldn't parse function name", src)?;
    let (src, _) = parse!(tok(chr('(')), "Expected '(' after function name", src)?;

    // Parse comma-separated arguments
    let (src, args) = parse!(parse_args, "Couldn't parse function arguments", src)?;

    let (src, _) = parse!(tok(chr(')')), "Expected ')' after function arguments", src)?;

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

fn parse_op<'s>() -> impl Parser<'s, &'s str> {
    preceded(
        choice!(
            keyword("+"),
            keyword("-"),
            keyword("*"),
            keyword("/"),
            keyword("^"),
            keyword("==")
        ),
        whitespace,
    )
}

pub fn parse_builtin_binop(src: Cursor) -> PResult<FunctionCall> {
    let (src, lhs) = parse!(primary, "Couldn't parse lhs expr", src)?;

    let (src, op) = parse!(parse_op(), "Couldn't parse operator expr", src)?;
    let (src, rhs) = parse!(primary, "Couldn't parse rhs expr", src)?;

    Ok((
        src,
        FunctionCall {
            name: op,
            args: vec![lhs, rhs],
            is_negated: false,
        },
    ))
}
