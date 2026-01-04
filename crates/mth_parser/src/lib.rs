pub mod parse_functions;

use graph_canvas::ops::Instruction;
use parser_lib::{
    cursor::Cursor,
    parse,
    types::{Context, PResult},
};

pub fn parse_program<'s>(s: &'s str) -> PResult<'s, Vec<Instruction>> {
    let src = Cursor::new(s);
    // let ctx = Context::default();
    // many1(parse!(expr(), "[Program] Expected expression", src))
    Ok((src, vec![]))
}
