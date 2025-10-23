use super::*;

pub fn parse_s_expr(src: Cursor) -> PResult<SExpr> {
    let (src, name) = parse!(tok(ident), "Could not parse s_expr name", src)?;
    let (src, args) = parse!(many0(parse_expr), "Could not parse s_expr args", src)?;
    Ok((src, SExpr { name, args }))
}
