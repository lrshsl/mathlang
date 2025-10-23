#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl<'s> {
    pub name: &'s str,
    pub params: Vec<Type>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Int,
    String,
    Bool,
}
