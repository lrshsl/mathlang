use super::*;

pub struct TypeDecl<'s> {
    pub name: &'s str,
    pub params: Vec<Type>,
}

pub enum Type {
    Int,
    String,
    Bool,
}

