use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SExpr<'s> {
    pub name: &'s str,
    pub args: Vec<Expr<'s>>,
}
