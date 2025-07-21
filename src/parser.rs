use nom::{
    bytes::{complete::take_while1, tag, take_while}, sequence::{delimited, preceded}, AsChar, Parser
};

use crate::{
    graph::ops::{Instruction, OP_X},
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

pub fn parse_func(s: &str) -> Result<(&str, Instruction), String> {
    define_parsers![
        ws = take_while(AsChar::is_space);
        ident = preceded(ws, take_while1(AsChar::is_alpha));
    ];
    let tok = |t| preceded(ws, t);
    let exact = |s| tok(tag(s));

    let (s, name) = parse!(ident, s)?;
    let (s, dep_var) = parse!(delimited(exact("("), ident, exact(")")), s)?;

    let (s, _) = parse!(exact("="), s)?;
    let (s, res_var) = parse!(delimited(exact("{"), ident, exact("}")), s)?;

    if dep_var == res_var {
        Ok((name, inst!(OP_X)))
    } else {
        Err("Not matching".to_string())
    }
}

pub fn parse(s: &str) -> Result<Vec<(&str, Instruction)>, String> {
    parse_func(s).map(|s| vec![s])
}
