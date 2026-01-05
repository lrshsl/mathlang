use parser_lib::types::BoxedParser;

use super::*;

pub fn s_expr_inner(src: Cursor) -> PResult<SExpr> {
    let (src, name) = parse!(tok(ident), "Could not parse s_expr name", src)?;
    let (src, args) = parse!(many0(primary), "Could not parse s_expr args", src)?;
    Ok((src, SExpr { name, args }))
}

pub fn s_expr_builtin_math(src: Cursor) -> PResult<SExpr> {
    let ops = "+-*/"
        .chars()
        .map(|op| Box::new(tok(chr(op))) as BoxedParser<'_, char>)
        .collect::<Vec<_>>();

    let (src, lhs) = parse!(primary, "Could not parse lhs expr", src)?;
    let (src, op) = parse!(choice_f(ops), "Could not parse operator expr", src)?;
    let (src, rhs) = parse!(primary, "Could not parse rhs expr", src)?;

    let name = match op {
        '+' => "__builtin__add",
        '-' => "__builtin__sub",
        '*' => "__builtin__mul",
        '/' => "__builtin__div",
        _ => unreachable!("Should cover all possible ops"),
    };

    Ok((
        src,
        SExpr {
            name,
            args: vec![lhs, rhs],
        },
    ))
}
