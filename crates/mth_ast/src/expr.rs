use super::*;

#[derive(Clone, PartialEq)]
pub enum Expr<'s> {
    FunctionCall(FunctionCall<'s>),
    Literal(Literal),
}

impl<'s> std::fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionCall(x) => x.fmt(f),
            Self::Literal(x) => x.fmt(f),
        }
    }
}

pub fn varref<'s>(name: &'s str) -> Expr<'s> {
    Expr::FunctionCall(FunctionCall { name, args: vec![] })
}

pub fn function_call<'s>(name: &'s str, args: Vec<Expr<'s>>) -> Expr<'s> {
    Expr::FunctionCall(FunctionCall { name, args })
}
