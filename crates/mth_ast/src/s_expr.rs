use std::ops::Neg;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall<'s> {
    pub name: &'s str,
    pub args: Vec<Expr<'s>>,
    pub is_negated: bool,
}

impl Neg for FunctionCall<'_> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            is_negated: !self.is_negated,
            ..self
        }
    }
}
