use crate::parser::{
    cursor::Cursor,
    types::{PError, PResult},
};

use super::*;

/// expr
///     : literal
///     | IDENT
///     | '(' exprList ')'
///     | expr expr
///     | expr op=Operator expr
///     ;
pub fn parse_expr<'s>(src: Cursor<'s>) -> PResult<'s, Expr<'s>> {
    // if let Ok((src, bin)) = parse_binary_expr(src.clone()) {
    // Ok((src, bin))
    // } else
    if let Ok((src, lit)) = parse_literal(src.clone()) {
        Ok((src, Expr::Literal(lit)))
    // }
    // else if let Ok((src, ref_)) = parse_ident(src.clone()) {
    //     Ok((src, ref_))
    } else {
        Err(PError {
            msg: "No parser succeeded".to_string(),
            ctx: src.ctx,
        })
    }
}
