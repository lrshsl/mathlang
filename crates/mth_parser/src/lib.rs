pub mod parse_functions;

use mth_ast::Module;
use parser_lib::{cursor::Cursor, types::PResult};

use crate::parse_functions::parse_module;

pub fn parse_program(s: &'_ str) -> PResult<'_, Module<'_>> {
    let src = Cursor::new(s);
    let (src, module_ast) = parse_module(src)?;
    Ok((src, module_ast))
}
