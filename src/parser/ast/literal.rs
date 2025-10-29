#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Str(String),
    Bool(bool),
}

pub fn int(x: i32) -> super::Expr<'static> {
    super::Expr::Literal(Literal::Int(x))
}
