use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mapping<'s> {
    pub name: &'s str,
    pub params: Vec<Param<'s>>,
    pub body: Expr<'s>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param<'s>(pub &'s str);
