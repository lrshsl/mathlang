use parser_lib::types::Parser;

use super::*;

/// var_assign
///	: IDENT '=' expr
///	;
pub fn parse_var_assign(src: Cursor) -> PResult<Function> {
    // Name
    let (src, name) = tok(ident)(src)?;

    // '='
    let (src, _) = tok(chr('='))(src)?;

    // Value
    let (src, value) = expr(src)?;

    Ok((
        src,
        Function {
            name,
            params: vec![],
            body: value,
        },
    ))
}

/// fn_decl
///	: IDENT '(' paramlist ')' '=' expr
///	;
///
/// paramlist
///	: (IDENT ',')* IDENT?
///	;
pub fn parse_fn_decl(src: Cursor) -> PResult<Function> {
    // Name
    let (src, name) = parse!(tok(ident), "Could not parse mapping name", src)?;

    // Params
    let (src, params) = parse!(
        between(paramlist(), tok(chr('(')), tok(chr('('))),
        "Couldn't parse params",
        src
    )?;

    // '='
    let (src, _) = parse!(tok(chr('=')), "Expected '=' in function declaration", src)?;

    // Body
    let (src, body) = expr(src)?;

    Ok((src, Function { name, params, body }))
}

pub fn paramlist<'s>() -> impl Parser<'s, Vec<Param<'s>>> {
    let param = pmap(tok(ident), |s| Param(s));
    delimited1(param, tok(chr(',')))
}
