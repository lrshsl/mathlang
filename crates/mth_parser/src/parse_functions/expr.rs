use parser_lib::{
    cursor::Cursor,
    pmatch,
    types::{PResult, Parser},
};

use crate::parse_functions::fn_call::{parse_builtin_binop, parse_fn_call, parse_s_expr};

use super::*;

/// expr
///     : expr op=Operator expr
///     | op=Operator expr+
///     | unary
///     | primary
///     ;
///
pub fn expr(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_expr] Couldn't match any subparser, tried `binop`, `s_expr`, `unary` and `primary`";
        parse_builtin_binop, x => Expr::FunctionCall(x);
        unary(), x => x;
        parse_s_expr, x => Expr::FunctionCall(x);
        primary, x => x;
    }
}

fn unary<'s>() -> impl Parser<'s, Expr<'s>> {
    pmap(preceded(primary, chr('-')), std::ops::Neg::neg)
}

/// primary
///     : Literal
///     | IDENT
///     | '(' expr ')'
pub fn primary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_primary] Couldn't match any subparser";
        literal, x => Expr::Literal(x);
        parse_fn_call, x => Expr::FunctionCall(x);
        tok(ident), x => varref(x);
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;
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
    fn parse_primary_function_call() {
        assert_primary("a(1, 2)", function_call("a", vec![int(1), int(2)]), "");
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
            "a(b(c))",
            function_call("a", vec![function_call("b", vec![varref("c")])]),
            "",
        );
    }
}
