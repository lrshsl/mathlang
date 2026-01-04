use parser_lib::{cursor::Cursor, parse, pmatch, types::PResult};

use crate::parse_functions::s_expr::s_expr_builtin_math;

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
        let Ok((new_src, expr)) = expr(src.clone()) else {
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
///     : expr op=Operator expr
///     | primary
///     | s_expr
///     ;
///
pub fn expr(src: Cursor) -> PResult<Expr> {
    // if let Ok((src, bin)) = parse_binary_expr(src.clone()) {
    // Ok((src, bin))
    // } else
    pmatch! {src; err = "[parse_expr] Could not match any subparser, tried `s_expr_inner` and `primary`";
        s_expr_inner, x => Expr::SExpr(x);
        s_expr_builtin_math, x => Expr::SExpr(x);
        primary, x => x;
    }
}

/// primary
///     : Literal
///     | IDENT
///     | '(' expr ')'
pub fn primary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_primary] Could not match any subparser";
        literal, x => Expr::Literal(x);
        tok(ident), x => varref(x);
        between(tok(expr), tok(chr('(')), tok(chr(')'))), x => x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helpers for convenience
    fn assert_primary(input: &str, expected: Expr, expected_rem: &str) {
        let (next, expr) = primary(Cursor::new(input)).expect("parse_primary failed");
        assert_eq!(expr, expected, "remainder: {}", next.remainder);
        assert_eq!(next.remainder, expected_rem);
    }

    fn assert_expr(input: &str, expected: Expr, expected_rem: &str) {
        let (next, expr) = expr(Cursor::new(input)).expect("parse_primary failed");
        assert_eq!(expr, expected, "remainder: {}", next.remainder);
        assert_eq!(next.remainder, expected_rem);
    }

    #[test]
    fn parse_primary_literal_int() {
        assert_primary("123", int(123), "");
    }

    #[test]
    fn parse_primary_identifier() {
        assert_primary("foo", varref("foo"), "");
    }

    #[test]
    fn parse_primary_s_expr() {
        assert_primary("(a 1 2)", s_expr("a", vec![int(1), int(2)]), "");
    }

    #[test]
    fn parse_primary_partial() {
        assert_primary("123abc", int(123), "abc");
    }

    #[test]
    fn parse_primary_invalid_input() {
        primary(Cursor::new("!")).expect_err("expected failure on invalid input");
    }

    #[test]
    fn parse_expr_nested() {
        assert_expr(
            "a (b c)",
            s_expr("a", vec![s_expr("b", vec![varref("c")])]),
            "",
        );
    }
}
