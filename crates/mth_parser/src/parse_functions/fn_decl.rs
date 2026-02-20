use parser_lib::types::Parser;

use super::*;

/// fn_def
///     : ident '(' (ident ',')* ident? ')' '=' expr
///     ;
pub fn parse_fn_decl(src: Cursor) -> PResult<Mapping> {
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

    Ok((src, Mapping { name, params, body }))
}

pub fn paramlist<'s>() -> impl Parser<'s, Vec<Param<'s>>> {
    let param = pmap(tok(ident), |s| Param(s));
    delimited1(param, tok(chr(',')))
}
