use parser_lib::{
    cursor::Cursor,
    pmatch,
    types::{PResult, Parser},
};

use super::*;

pub fn expr(src: Cursor) -> PResult<Expr> {
    logical_or(src)
}

// logical `or` (lowest precedence)
fn logical_or(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(logical_and, tok(keyword(s))), logical_or);

    pmatch!(src; err = "[logical_or] expected expression";

        bin_op("or"), exprs => function_call("or", exprs);
        logical_and, x => x;
    )
}

// logical `and`
fn logical_and(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(comparison, tok(keyword(s))), logical_and);

    pmatch!(src; err = "[logical_and] expected expression";

        bin_op("and"), exprs => function_call("and", exprs);
        comparison, x => x;
    )
}

// comparison operators (`==`, `<`, `>`, `<=`, `>=`)
fn comparison(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(term, tok(keyword(s))), comparison);

    pmatch!(src; err = "[comparison] expected expression";

        bin_op("=="), exprs => function_call("==", exprs);
        bin_op("<"), exprs => function_call("<", exprs);
        bin_op(">"), exprs => function_call(">", exprs);
        bin_op("<="), exprs => function_call("<=", exprs);
        bin_op(">="), exprs => function_call(">=", exprs);
        term, x => x;
    )
}

// arithmetic term (`+`, `-`)
fn term(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(factor, tok(keyword(s))), term);

    pmatch!(src; err = "[term] expected expression";

        bin_op("+"), exprs => function_call("+", exprs);
        bin_op("-"), exprs => function_call("-", exprs);

        factor, x => x;
    )
}

// factor (`*`, `/`, and implicit multiplication)
fn factor(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(power, tok(keyword(s))), factor);

    pmatch!(src; err = "[factor] expected expression";

        bin_op("*"), exprs => function_call("*", exprs);
        bin_op("/"), exprs => function_call("/", exprs);

        // implicit multiplication: two primary expressions side-by-side
        and_then(primary, factor), exprs => function_call("*", exprs);

        power, x => x;
    )
}

// power (`^`) (highest binary precedence)
fn power(src: Cursor) -> PResult<Expr> {
    let bin_op = |s| and_then(terminated(unary, tok(keyword(s))), power);

    pmatch!(src; err = "[power] expected expression";

        bin_op("^"), exprs => function_call("^", exprs);
        unary, e => e;
    )
}

fn unary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_prefix] Couldn't match any prefix expression";

        // Unary negation - must come before regular expressions
        preceded(chr('-'), unary), x => -x;
        preceded(chr('+'), unary), x => x;

        // Regular primary expressions
        primary, x => x;
    }
}

/// primary
///     : Literal
///     | IDENT
///     | '(' expr ')'
///
/// Supports implicit multiplication: "2 x" -> "2 * x"
pub fn primary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_primary] Couldn't match any subparser";
        // '(' expr ')'
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;

        // Regular primary expressions
        parse_fn_call, x => Expr::FunctionCall(x);
        tok(ident), x => varref(x);
        literal, x => Expr::Literal(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helpers for convenience
    fn assert_expr(input: &str, expected: Expr, expected_rem: &str) {
        let (next, expr) = expr(Cursor::new(input)).expect("parse_expr failed");
        assert_eq!(expr, expected, "remainder: {}", next.remainder);
        assert_eq!(next.remainder, expected_rem);
    }

    #[test]
    fn parse_unary_neg_literal() {
        assert_expr("-5", -int(5), "");
    }

    #[test]
    fn parse_unary_neg_variable() {
        assert_expr("-x", -varref("x"), "");
    }

    #[test]
    fn parse_implicit_multiply_literal_variable() {
        assert_expr("2 x", function_call("*", vec![int(2), varref("x")]), "");
    }

    #[test]
    fn parse_implicit_multiply_variable_literal() {
        assert_expr("x 5", function_call("*", vec![varref("x"), int(5)]), "");
    }
}
