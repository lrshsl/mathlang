use parser_lib::{combinators::preceded, primitives::keyword, types::BoxedParser};

use super::*;

pub fn parse_function_call(src: Cursor) -> PResult<FunctionCall> {
    let (src, name) = parse!(tok(ident), "Could not parse function name", src)?;
    let (src, _) = parse!(tok(chr('(')), "Expected '(' after function name", src)?;

    // Parse comma-separated arguments
    let (src, args) = parse!(
        parse_comma_separated_args,
        "Could not parse function arguments",
        src
    )?;

    let (src, _) = parse!(tok(chr(')')), "Expected ')' after function arguments", src)?;

    Ok((src, FunctionCall { name, args }))
}

fn parse_comma_separated_args(src: Cursor) -> PResult<Vec<Expr>> {
    // Try to parse the first argument
    match expr(src.clone()) {
        Ok((src, first)) => {
            // Parse remaining comma-separated arguments
            let mut args = vec![first];
            let mut current_src = src;

            loop {
                match preceded(expr, tok(chr(',')))(current_src.clone()) {
                    Ok((new_src, arg)) => {
                        args.push(arg);
                        current_src = new_src;
                    }
                    Err(_) => break,
                }
            }

            Ok((current_src, args))
        }
        Err(_) => {
            // No arguments
            Ok((src, vec![]))
        }
    }
}

pub fn function_call_builtin_math(src: Cursor) -> PResult<FunctionCall> {
    let (src, lhs) = parse!(primary, "Could not parse lhs expr", src)?;

    // Fall back to original single-character operators
    let ops = ["+", "-", "*", "/", "^", "=="]
        .into_iter()
        .map(|op| Box::new(tok(keyword(op))) as BoxedParser<'_, &str>)
        .collect::<Vec<_>>();

    let (src, op) = parse!(choice_f(ops), "Could not parse operator expr", src)?;
    let (src, rhs) = parse!(primary, "Could not parse rhs expr", src)?;

    let name = match op {
        "+" => "__builtin__add",
        "-" => "__builtin__sub",
        "*" => "__builtin__mul",
        "/" => "__builtin__div",
        "^" => "pow",
        "==" => "__builtin__eq",
        _ => unreachable!("Should cover all possible ops"),
    };

    Ok((
        src,
        FunctionCall {
            name,
            args: vec![lhs, rhs],
        },
    ))
}
