use nom::{
    AsChar, Parser,
    bytes::{complete::take_while1, tag},
    sequence::preceded,
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

pub fn parse_func(s: &str) -> Result<(&str, Instruction), String> {
    define_parsers![
        ws = take_while1(AsChar::is_space);
        ident = preceded(ws, take_while1(AsChar::is_alpha));
    ];

    // let (_, name) = ident
    //     .parse(s)
    //     .map_err(|e| format!("Incomplete input: expected ident: {e}"))?;

    // let (_, _) = preceded(ws, tag("="))
    //     .parse(s)
    //     .map_err(|e| format!("Incomplete input: expected '=': {e}"))?;

    Ok(("", inst!(OP_X)))
}

// pub fn parse(s: &str) -> Result<Vec<Instruction>, String> {
//     Ok((name, vec![
//         inst!(OP_INPUT_X, 0., 0.),
//     ]))
// }
