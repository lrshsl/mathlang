use super::*;

pub fn parse_type_decl(src: Cursor) -> PResult<TypeDecl> {
    let (src, name) = parse!(tok(ident), "Could not parse type name", src)?;
    let (src, _) = parse!(tok(keyword("::")), "Could not find '::'", src)?;
    let (src, params) = parse!(some(parse_type), "Could not parse type params", src)?;
    Ok((src, TypeDecl { name, params }))
}

pub fn parse_type(src: Cursor) -> PResult<Type> {
    pmatch! {src; err = "[parse_type]";
        tok(keyword("int")), _ => Type::Int;
        tok(keyword("string")), _ => Type::String;
        tok(keyword("bool")), _ => Type::Bool;
    }
}
