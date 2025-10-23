use crate::{
    parse,
    parser::{
        cursor::Cursor,
        types::{PError, PResult},
    },
};

use super::*;

/// exprList
///     : (expr (';' expr)* ';'?)?
///     ;
///
pub fn parse_expr_list<'s>(mut src: Cursor<'s>) -> PResult<'s, Vec<Expr<'s>>> {
    let mut exprs = Vec::new();
    loop {
        while let Ok((new_src, _)) = parse!(chr(';'), "Expected ';'", src.clone()) {
            src = new_src;
        }
        let Ok((new_src, expr)) = parse_expr(src.clone()) else {
            // TODO
            break;
        };
        exprs.push(expr);
        src = new_src;
    }
    Ok((src, exprs))
}

/// expr
///     : literal
///     | IDENT
///     | '(' exprList ')'
///     | expr expr
///     | expr op=Operator expr
///     ;
///
pub fn parse_expr<'s>(src: Cursor<'s>) -> PResult<'s, Expr<'s>> {
    // if let Ok((src, bin)) = parse_binary_expr(src.clone()) {
    // Ok((src, bin))
    // } else
    if let Ok((src, lit)) = parse_literal(src.clone()) {
        Ok((src, Expr::Literal(lit)))
    } else if let Ok((src, s_expr)) = parse_s_expr(src.clone()) {
        Ok((src, Expr::SExpr(s_expr)))
    } else {
        Err(PError {
            msg: "No parser succeeded".to_string(),
            ctx: src.ctx,
        })
    }
}
