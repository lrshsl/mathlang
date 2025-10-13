#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Str(String),
    Bool(bool),
}
