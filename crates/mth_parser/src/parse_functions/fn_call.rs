use parser_lib::types::BoxedParser;

use super::*;

pub fn parse_fn_call(src: Cursor) -> PResult<FunctionCall> {
    let parse_args = delimited1(tok(expr), tok(chr(',')));

    let (src, name) = parse!(tok(ident), "Could not parse function name", src)?;
    let (src, _) = parse!(tok(chr('(')), "Expected '(' after function name", src)?;

    // Parse comma-separated arguments
    let (src, args) = parse!(parse_args, "Could not parse function arguments", src)?;

    let (src, _) = parse!(tok(chr(')')), "Expected ')' after function arguments", src)?;

    Ok((src, FunctionCall { name, args }))
}

pub fn parse_builtin_binop(src: Cursor) -> PResult<FunctionCall> {
    let (src, lhs) = parse!(primary, "Could not parse lhs expr", src)?;

    let ops = ["+", "-", "*", "/", "^", "=="]
        .into_iter()
        .map(|op| Box::new(tok(keyword(op))) as BoxedParser<'_, &str>)
        .collect::<Vec<_>>();

    let (src, op) = parse!(choice_f(ops), "Could not parse operator expr", src)?;
    let (src, rhs) = parse!(primary, "Could not parse rhs expr", src)?;

    let name = match op {
        "+" => "__builtin__add",
        "-" => "__builtin__sub",
        "*" => "__builtin__mul",
        "/" => "__builtin__div",
        "^" => "pow",
        "==" => "__builtin__eq",
        _ => unreachable!("Should cover all possible ops"),
    };

    Ok((
        src,
        FunctionCall {
            name,
            args: vec![lhs, rhs],
        },
    ))
}
