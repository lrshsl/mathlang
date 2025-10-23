use crate::{
    parse,
    parser::{
        cursor::Cursor,
        types::{PError, PResult},
    },
    pmatch,
};

use super::*;

/// exprList
///     : (expr (';' expr)* ';'?)?
///     ;
///
fn parse_expr_list<'s>(mut src: Cursor<'s>) -> PResult<'s, Vec<Expr<'s>>> {
    let mut exprs = Vec::new();
    loop {
        while let Ok((new_src, _)) = parse!(chr(';'), "Expected ';'", src.clone()) {
            src = new_src;
        }
        let Ok((new_src, expr)) = parse_expr(src.clone()) else {
            // TODO
            eprintln!("Error in [parse_expr_list], recovering");
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
pub fn parse_expr(src: Cursor) -> PResult<Expr> {
    // if let Ok((src, bin)) = parse_binary_expr(src.clone()) {
    // Ok((src, bin))
    // } else
    pmatch! {src; err = "[parse_expr] Could not match any subparser";
        parse_s_expr, x => Expr::SExpr(x);
        parse_primary, x => x;
    }
}

pub fn parse_primary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_primary] Could not match any subparser";
        parse_literal, x => Expr::Literal(x);
        ident, x => varref(x);
        between(parse_expr, chr('('), chr(')')), x => x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper for convenience
    fn assert_primary(input: &str, expected: Expr, expected_rem: &str) {
        let (next, expr) = parse_primary(Cursor::new(input)).expect("parse_primary failed");
        assert_eq!(expr, expected);
        assert_eq!(next.remainder, expected_rem);
    }

    #[test]
    fn parse_primary_literal_int() {
        assert_primary("123", Expr::Literal(Literal::Int(123)), "");
    }

    #[test]
    fn parse_primary_identifier() {
        assert_primary("foo", varref("foo"), "");
    }

    #[test]
    fn parse_primary_paren_expr() {
        assert_primary("(42)", Expr::Literal(Literal::Int(42)), "");
    }

    #[test]
    fn parse_primary_nested() {
        assert_primary(
            "a (b c)",
            s_expr("a", vec![s_expr("b", vec![varref("c")])]),
            "",
        );
    }

    #[test]
    fn parse_primary_partial() {
        assert_primary("123abc", Expr::Literal(Literal::Int(123)), "abc");
    }

    #[test]
    fn parse_primary_invalid_input() {
        let err = parse_primary(Cursor::new("!")).expect_err("expected failure on invalid input");
        assert!(err.msg.contains("No parser succeeded"));
    }
}
