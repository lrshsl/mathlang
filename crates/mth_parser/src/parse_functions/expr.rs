use parser_lib::{cursor::Cursor, helpers::whitespace, pmatch, types::PResult};

use crate::parse_functions::fn_call::parse_op;
use mth_ast::FunctionCall;

use super::*;

/// Main expression parser using Pratt parsing for proper operator precedence
pub fn expr(src: Cursor) -> PResult<Expr> {
    parse_expression_with_precedence(src, 0)
}

fn parse_expression_with_precedence(src: Cursor, min_prec: u8) -> PResult<Expr> {
    // Parse left side (could be unary or primary)
    let (mut src, mut left) = parse_prefix_expression(src)?;

    // Parse binary operators while precedence >= min_prec
    while let Ok((next_src, (op, prec, right_assoc))) = parse_binop(src.clone()) {
        if prec < min_prec {
            break;
        }

        // Consume the operator
        src = next_src;

        // Parse right side with higher precedence for left-associative operators
        let next_min_prec = if right_assoc { prec } else { prec + 1 };
        let (next_src, right) = parse_expression_with_precedence(src, next_min_prec)?;

        left = Expr::FunctionCall(FunctionCall {
            name: op,
            args: vec![left, right],
            is_negated: false,
        });

        src = next_src;
    }

    Ok((src, left))
}

fn parse_prefix_expression(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_prefix] Couldn't match any prefix expression";

        // Unary negation - must come before regular expressions
        preceded(chr('-'), parse_prefix_expression), x => -x;
        preceded(chr('+'), parse_prefix_expression), x => x;

        // Parenthesized expressions (should be tried before implicit multiplication)
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;

        // Function call
        parse_fn_call, x => Expr::FunctionCall(x);

        // Regular primary expressions (includes implicit multiplication)
        primary, x => x;
    }
}

fn parse_binop(src: Cursor) -> PResult<(&'static str, u8, bool)> {
    let op_parser = tok(some(satisfy(|ch| !ch.is_whitespace())));
    let (src, op) = parse!(op_parser, "[parse_binop] No binop matched", src)?;

    let opinfo = get_operator_info()
        .iter()
        .filter(|opinfo| op.iter().copied().eq(opinfo.0.chars()))
        .next()
        .cloned()
        .ok_or(PError {
            msg: "[parse_binop] No binop matched".to_owned(),
            ctx: src.ctx.clone(),
        })?;

    Ok((src, opinfo))
}

fn get_operator_info() -> &'static [(&'static str, u8, bool)] {
    // (operator, precedence, is_right_associative)
    // Higher numbers = higher precedence
    &[
        // Logical ops (lowest precedence)
        ("and", 1, false),
        ("or", 2, false),
        // Bitwise ops
        ("bitwise_and", 3, false),
        ("bitwise_xor", 4, false),
        ("bitwise_or", 5, false),
        // Comparison ops
        ("==", 6, false),
        ("!=", 6, false),
        ("<=", 6, false),
        ("<", 6, false),
        (">=", 6, false),
        (">", 6, false),
        // Addition and subtraction (as binary ops)
        ("+", 7, false),
        ("-", 7, false),
        // Multiplication and division
        ("*", 8, false),
        ("/", 8, false),
        // Exponentiation (right-associative)
        ("^", 9, true),
    ]
}

/// primary
///     : Literal
///     | IDENT
///     | '(' expr ')'
///
/// Supports implicit multiplication: "2 x" -> "2 * x"
pub fn primary(src: Cursor) -> PResult<Expr> {
    pmatch! {src; err = "[parse_primary] Couldn't match any subparser";
        // Try implicit multiplication first (literal/identifier followed by expression)
        try_implicit_multiplication, x => x;
        // Regular primary expressions
        literal, x => Expr::Literal(x);
        parse_fn_call, x => Expr::FunctionCall(x);
        tok(non_keyword_ident), x => varref(x);
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;
    }
}

pub fn non_keyword_ident(src: Cursor<'_>) -> PResult<'_, &'_ str> {
    let (src, id) = ident(src)?;
    if ["and", "or", "bitwise_and", "bitwise_xor", "bitwise_or"].contains(&id) {
        return Err(PError {
            msg: format!("Cannot use {id} as identifier since it is a hard keyword"),
            ctx: src.ctx.clone(),
        });
    }
    Ok((src, id))
}

// Try to parse implicit multiplication: "2 x" -> "2 * x"
fn try_implicit_multiplication(src: Cursor) -> PResult<Expr> {
    // Parse a literal or identifier first
    let (src, left) = pmatch! {src; err = "[implicit_multiply] Expected literal or identifier";
        literal, x => Expr::Literal(x);
        tok(non_keyword_ident), x => varref(x);
    }?;

    // Skip whitespace - required for implicit multiplication
    let src = match whitespace(src.clone()) {
        Ok((new_src, _)) => new_src,
        Err(_) => return Ok((src, left)), // No whitespace, don't do implicit multiplication
    };

    // Check if next token can be multiplied implicitly (literal, identifier, or '(')
    // But NOT if it looks like a function call (identifier followed by '(' without space)
    let can_multiply = match src.clone() {
        test_src => {
            // Check if it starts with '(' (for parentheses)
            if test_src.remainder.starts_with('(') {
                true
            } else if literal(test_src.clone()).is_ok() {
                // Check for literal
                true
            } else if let Ok((after_ident, _)) = tok(non_keyword_ident)(test_src.clone()) {
                // Check for identifier, but make sure it's not followed immediately by '('
                // If the identifier is followed immediately by '(', it's a function call, not implicit multiplication
                !after_ident.remainder.starts_with('(')
            } else {
                false
            }
        }
    };

    if !can_multiply {
        return Ok((src, left));
    }

    // Parse the right side (but only primary expressions, not full expressions)
    let (src, right) = pmatch! {src; err = "[implicit_multiply] Expected right operand";
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;
        literal, x => Expr::Literal(x);
        tok(non_keyword_ident), x => varref(x);
    }?;

    Ok((
        src,
        Expr::FunctionCall(FunctionCall {
            name: "*",
            args: vec![left, right],
            is_negated: false,
        }),
    ))
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
