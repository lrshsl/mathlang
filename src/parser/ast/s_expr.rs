use super::*;

pub struct SExpr<'s> {
    pub name: &'s str,
    pub args: Vec<Expr<'s>>,
}
