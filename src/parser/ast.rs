pub enum Expr<'s> {
    Nr(f32),
    Ref(&'s str),
}
