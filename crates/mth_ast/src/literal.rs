use std::ops::Neg;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Bool(bool),
}

impl Neg for Literal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(v) => Self::Int(-v),
            Self::Float(v) => Self::Float(-v),
            Self::Bool(v) => Self::Bool(!v),
        }
    }
}

pub fn int(x: i32) -> super::Expr<'static> {
    super::Expr::Literal(Literal::Int(x))
}
