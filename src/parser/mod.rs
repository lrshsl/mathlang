pub mod ast;
pub mod cursor;
pub mod parse_functions;
mod parser_lib;
pub mod types;

use crate::{
    graph::ops::Instruction,
    parser::{cursor::Cursor, types::PResult},
};

pub fn parse_program<'s>(s: &'s str) -> PResult<'s, Vec<Instruction>> {
    let src = Cursor::new(s);
    // let ctx = HashMap::new();
    // many1(parse!(expr(), "[Program] Expected expression", src))
    //
    //
    Ok((src, vec![]))
}
