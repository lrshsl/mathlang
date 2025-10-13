use super::*;

pub struct Mapping<'s> {
    pub name: &'s str,
    pub params: Vec<Param<'s>>,
    pub body: Expr<'s>,
}

pub struct Param<'s>(pub &'s str);
