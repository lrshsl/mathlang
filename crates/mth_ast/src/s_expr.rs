use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'s> {
    pub name: &'s str,
    pub args: Vec<Expr<'s>>,
}
