use nom::{
    AsChar, Parser,
    bytes::{complete::take_while1, tag, take_while},
    sequence::{delimited, preceded},
};

use crate::{
    graph::ops::{Instruction, OP_X, OP_X_POLY},
    inst,
};

macro_rules! define_parsers {
    [ $( $name:ident = $body:expr );* $(;)? ] => {
        $(
            fn $name(s: &str) -> nom::IResult<&str, &str> {
                $body.parse(s)
            }
        )*
    };
}

macro_rules! parse {
    ($p:expr, $s:expr) => {
        $p.parse($s).map_err(|e| {
            format!(
                "In {e}\n\tUnexpected token: Expected {}, found: {}..\n",
                stringify!(p),
                $s.chars().take(5).collect::<String>()
            )
        })
    };
}

define_parsers![
    ws = take_while(AsChar::is_space);
    ident = preceded(ws, take_while1(AsChar::is_alpha));
];

pub struct Polynom<'s> {
    var: &'s str,
    factor: f32,
    exponent: f32,
}

pub fn parse_exprs(s: &'_ str) -> Result<(&'_ str, Polynom<'_>), String> {
    let tok = |t| preceded(ws, t);
    let exact = |s| tok(tag(s));

    let (s, factor) = parse!(preceded(ws, nom::number::float()), s)?;
    let (s, var) = parse!(preceded(ws, ident), s)?;
    let (s, exponent) = parse!(preceded(exact("**"), nom::number::float()), s)?;
    Ok((
        s,
        Polynom {
            var,
            factor,
            exponent,
        },
    ))
}

pub fn parse_func(s: &str) -> Result<(&str, Instruction), String> {
    let tok = |t| preceded(ws, t);
    let exact = |s| tok(tag(s));

    let (s, name) = parse!(ident, s)?;
    let (s, dep_var) = parse!(delimited(exact("("), ident, exact(")")), s)?;

    let (s, _) = parse!(exact("="), s)?;
    let (s, _) = parse!(exact("{"), s)?;
    let (s, polynom) = parse_exprs(s).map_err(|e| format!("Invalid polynom: {e}"))?;
    let (s, _) = parse!(exact("}"), s)?;

    if dep_var == polynom.var {
        Ok((name, inst!(OP_X_POLY, polynom.factor, polynom.exponent)))
    } else {
        Err("Not matching".to_string())
    }
}

pub fn parse(s: &str) -> Result<Vec<(&str, Instruction)>, String> {
    parse_func(s).map(|s| vec![s])
}
