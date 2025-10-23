use super::*;

// pub struct Mapping<'s> {
//     pub name: &'s str,
//     pub params: Vec<Param<'s>>,
//     pub body: Expr<'s>,
// }

// pub struct Param<'s>(pub &'s str);

pub fn parse_mapping(src: Cursor) -> PResult<Mapping> {
    let (src, name) = parse!(tok(ident), "Could not parse mapping name", src.clone())?;
    let (src, params) = parse!(many0(parse_param), "Could not parse params", src.clone())?;
    let (src, _) = parse!(tok(keyword("->")), "Could not find '->'", src.clone())?;
    let (src, body) = parse_expr(src.clone())?;
    Ok((src, Mapping { name, params, body }))
}

pub fn parse_param(src: Cursor) -> PResult<Param> {
    let (src, name) = parse!(tok(ident), "Could not parse param name", src.clone())?;
    Ok((src, Param(name)))
}
