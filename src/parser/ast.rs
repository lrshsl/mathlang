use crate::{
    graph::ops::{Instruction, OP_CONST, OP_MUL, OP_X},
    inst,
};

pub enum Expr<'s> {
    Nr(f32),
    Ref(&'s str),
    Minus(Box<Expr<'s>>),
}

impl<'s> Expr<'s> {
    pub fn compile(&self) -> Vec<Instruction> {
        match self {
            Self::Minus(val) => {
                let mut v = val.compile();
                v.push(inst!(OP_MUL, -1.0));
                v
            }
            Self::Nr(n) => vec![inst!(OP_CONST, *n)],
            Self::Ref("x") => vec![inst!(OP_X)],
            Self::Ref(r) => panic!("Undefined reference: {r}"),
        }
    }
}
