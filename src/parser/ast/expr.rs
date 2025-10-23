use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'s> {
    SExpr(SExpr<'s>),
    Ref(&'s str),
    Literal(Literal),
}
