
pub struct Func {
    var_name: String,
    term: Term,
}

pub enum Term {
    Add(Term, Term),
    Sub(Term, Term),
    Mul(Term, Term),
    Div(Term, Term),
}

