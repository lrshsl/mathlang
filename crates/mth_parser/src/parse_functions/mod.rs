pub(self) use mth_ast::*;
pub(self) use parser_lib::{
    combinators::*,
    cursor::Cursor,
    helpers::*,
    parse, pmatch,
    primitives::*,
    types::{PError, PResult},
};

mod module;
pub use module::parse_module;

mod top_level;
pub use top_level::parse_top_level;

mod expr;
pub use expr::{expr, primary};

mod type_decl;
pub use type_decl::parse_type_decl;

mod fn_decl;
pub use fn_decl::parse_fn_decl;

mod fn_call;
pub use fn_call::{parse_fn_call, parse_op};

mod literal;
pub use literal::literal;

#[cfg(test)]
mod tests;
