use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel<'s> {
    TypeDecl(TypeDecl<'s>),
    Function(Function<'s>),
    Expr(Expr<'s>),
}
