use super::*;

// pub struct TypeDecl<'s> {
//     pub name: &'s str,
//     pub params: Vec<Type>,
// }

// pub enum Type {
//     Int,
//     String,
//     Bool,
// }

pub fn parse_type_decl(src: Cursor) -> PResult<TypeDecl> {
    let (src, name) = parse!(tok(ident), "Could not parse type name", src)?;
    let (src, _) = parse!(tok(keyword("::")), "Could not find '::'", src)?;
    let (src, params) = parse!(some(parse_type), "Could not parse type params", src)?;
    Ok((src, TypeDecl { name, params }))
}

pub fn parse_type(src: Cursor) -> PResult<Type> {
    if let Ok((src, _)) = parse!(tok(keyword("int")), "int", src.clone()) {
        Ok((src, Type::Int))
    } else if let Ok((src, _)) = parse!(tok(keyword("string")), "string", src.clone()) {
        Ok((src, Type::String))
    } else if let Ok((src, _)) = parse!(tok(keyword("bool")), "bool", src.clone()) {
        Ok((src, Type::Bool))
    } else {
        todo!()
    }
}
