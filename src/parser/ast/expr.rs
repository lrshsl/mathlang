use super::*;

#[derive(Clone, PartialEq)]
pub enum Expr<'s> {
    SExpr(SExpr<'s>),
    Literal(Literal),
}

impl<'s> std::fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SExpr(x) => x.fmt(f),
            Self::Literal(x) => x.fmt(f),
        }
    }
}

pub fn varref<'s>(name: &'s str) -> Expr<'s> {
    Expr::SExpr(SExpr { name, args: vec![] })
}

pub fn s_expr<'s>(name: &'s str, args: Vec<Expr<'s>>) -> Expr<'s> {
    Expr::SExpr(SExpr { name, args })
}
