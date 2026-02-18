use parser_lib::{cursor::Cursor, helpers::whitespace, pmatch, types::PResult};

use crate::parse_functions::fn_call::{parse_op, parse_s_expr};
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
    while let Some((op, prec, right_assoc)) = peek_binary_operator(src.clone()) {
        if prec < min_prec {
            break;
        }

        // Consume the operator
        let (next_src, _) = parse!(parse_op(), "Expected operator", src)?;

        // Parse right side with higher precedence for left-associative operators
        let next_min_prec = if right_assoc { prec } else { prec + 1 };
        let (next_src, right) = parse_expression_with_precedence(next_src, next_min_prec)?;

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
        // Regular primary expressions (includes implicit multiplication)
        primary, x => x;
        // S-expressions with function names (like "+", "*", etc.) - only if no implicit multiplication
        parse_s_expr, x => Expr::FunctionCall(x);
    }
}

fn peek_binary_operator(src: Cursor) -> Option<(&'static str, u8, bool)> {
    // Look ahead to see if we have a binary operator and its precedence
    let test_src = src.clone();

    // Skip whitespace if present
    let test_src = match whitespace(test_src.clone()) {
        Ok((next_src, _)) => next_src,
        Err(_) => test_src,
    };

    // Check for each operator
    for (op, precedence, right_assoc) in get_operator_info() {
        if let Ok((_, _)) = keyword(op)(test_src.clone()) {
            return Some((op, *precedence, *right_assoc));
        }
    }

    None
}

fn get_operator_info() -> &'static [(&'static str, u8, bool)] {
    // (operator, precedence, is_right_associative)
    // Higher numbers = higher precedence
    &[
        // Comparison operators (lowest precedence)
        ("==", 1, false),
        ("!=", 1, false),
        ("<=", 1, false),
        ("<", 1, false),
        (">=", 1, false),
        (">", 1, false),
        // Addition and subtraction
        ("+", 2, false),
        ("-", 2, false),
        // Multiplication and division
        ("*", 3, false),
        ("/", 3, false),
        // Exponentiation (right-associative)
        ("^", 4, true),
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
        tok(ident), x => varref(x);
        between(expr, tok(chr('(')), tok(chr(')'))), x => x;
    }
}

// Try to parse implicit multiplication: "2 x" -> "2 * x"
fn try_implicit_multiplication(src: Cursor) -> PResult<Expr> {
    // Parse a literal or identifier first
    let (src, left) = pmatch! {src; err = "[implicit_multiply] Expected literal or identifier";
        literal, x => Expr::Literal(x);
        tok(ident), x => varref(x);
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
            } else if let Ok((after_ident, _)) = tok(ident)(test_src.clone()) {
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
        tok(ident), x => varref(x);
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
