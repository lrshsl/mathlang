use parser_lib::Parser;

use super::*;

/// fn_def
///     : ident '(' (ident ',')* ident? ')' '=' expr
///     ;
pub fn parse_mapping(src: Cursor) -> PResult<Mapping> {
    // Name
    let (src, name) = parse!(tok(ident), "Could not parse mapping name", src)?;

    // Params
    let (src, _) = parse!(tok(chr('(')), "Expected '(' in function declaration", src)?;
    let (src, params) = parse!(
        delimited1(parse_param(), tok(chr(','))),
        "Could not parse params",
        src
    )?;
    let (src, _) = parse!(tok(chr(')')), "Expected ')' in function declaration", src)?;

    // '='
    let (src, _) = parse!(tok(chr('=')), "Expected '=' in function declaration", src)?;

    // Body
    let (src, body) = expr(src)?;

    Ok((src, Mapping { name, params, body }))
}

pub fn parse_param<'s>() -> Parser!['s, Param] {
    pmap(tok(ident), |s| Param(s))
}
