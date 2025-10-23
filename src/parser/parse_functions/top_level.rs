use super::*;

pub fn parse_top_level(src: Cursor) -> PResult<TopLevel> {
    if let Ok((src, v)) = parse!(
        terminated(parse_type_decl, chr(';')),
        "type_decl",
        src.clone()
    ) {
        Ok((src, TopLevel::TypeDecl(v)))
    } else if let Ok((src, v)) = parse!(terminated(parse_mapping, chr(':')), "mapping", src.clone())
    {
        Ok((src, TopLevel::MapImpl(v)))
    } else if let Ok((src, v)) = parse!(terminated(parse_expr, chr(';')), "expr", src.clone()) {
        Ok((src, TopLevel::Expr(v)))
    } else {
        todo!()
    }
}
