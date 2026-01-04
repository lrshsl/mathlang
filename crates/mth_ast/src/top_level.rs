use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel<'s> {
    TypeDecl(TypeDecl<'s>),
    MapImpl(Mapping<'s>),
    Expr(Expr<'s>),
}
