use super::*;

pub enum Expr<'s> {
    SExpr(SExpr<'s>),
    Ref(&'s str),
    Literal(Literal),
}
