pub use crate::{parse, pmatch};

pub(self) use crate::parser::{
    ast::*,
    cursor::Cursor,
    parser_lib::{combinators::*, helpers::*, primitives::*},
    types::{PError, PResult},
};

mod module;
pub use module::parse_module;

mod top_level;
pub use top_level::parse_top_level;

mod expr;
pub use expr::{parse_expr, parse_primary};

mod type_decl;
pub use type_decl::parse_type_decl;

mod mapping;
pub use mapping::parse_mapping;

mod s_expr;
pub use s_expr::parse_s_expr;

mod literal;
pub use literal::parse_literal;

#[cfg(test)]
mod tests;
