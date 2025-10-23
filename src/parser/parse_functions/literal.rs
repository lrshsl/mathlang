use crate::{choice, parse};

use super::*;

pub fn parse_literal(src: Cursor) -> PResult<Literal> {
    pmatch! {src; err = "[parse_literal]";
        string, x => Literal::Str(x);
        boolean, x => Literal::Bool(x);
    }
    .or_else(|e| {
        if let Ok((int_remainder, int)) = int(src.clone()) {
            //
            // Check if looks like float
            let next_char = int_remainder.remainder.chars().next();
            if next_char == Some('.') || next_char == Some('e') {
                let (float_remainder, float) = match float(src.clone()) {
                    Ok(v) => v,
                    Err(e) => {
                        return Err(PError {
                            msg: format!(
                                "[parse_literal] Could not parse float: {}\n{}",
                                src.remainder, e.msg
                            ),
                            ctx: src.ctx,
                        });
                    }
                };
                return Ok((float_remainder, Literal::Float(float)));
            }

            return Ok((int_remainder, Literal::Int(int)));
        }
        Err(e)
    })
}

fn string(src: Cursor) -> PResult<String> {
    let (src, _) = parse!(chr('"'), "Expected opening quote", src)?;
    let (src, content) = parse!(some(satisfy(|ch| ch != '"')), "Expected string", src)?;
    let (src, _) = parse!(chr('"'), "Expected closing quote", src)?;
    Ok((src, String::from_iter(content.into_iter())))
}

fn int(src: Cursor) -> PResult<i32> {
    let (src, int) = parse!(tok(some(digit(10))), "Could not parse int", src)?;
    Ok((
        src,
        String::from_iter(int.into_iter())
            .parse::<i32>()
            .expect("Failed to parse int"),
    ))
}

fn float(src: Cursor) -> PResult<f64> {
    let (src, int_part) = parse!(some(digit(10)), "Expected digits before '.'", src)?;
    let (src, _) = parse!(chr('.'), "Expected '.' in float", src)?;
    let (src, frac_part) = parse!(some(digit(10)), "Expected digits after '.'", src)?;

    let mut s: String = int_part
        .into_iter()
        .chain(std::iter::once('.'))
        .chain(frac_part)
        .collect();

    // Optional exponent part
    let src_after_exp = src.clone();
    if let Ok((src_e, _)) =
        chr('e')(src_after_exp.clone()).or_else(|_| chr('E')(src_after_exp.clone()))
    {
        let (src_e, sign) = optional(choice!(chr('+'), chr('-')))(src_e)?;
        let (src_e, exp_digits) = some(digit(10))(src_e)?;

        s.push('e');
        if let Some(sign) = sign {
            s.push(sign);
        }
        s.extend(exp_digits);

        return match s.parse::<f64>() {
            Ok(f) => Ok((src_e, f)),
            Err(_) => Err(PError {
                msg: format!("Invalid float literal: {}", s),
                ctx: src_e.ctx,
            }),
        };
    }

    match s.parse::<f64>() {
        Ok(f) => Ok((src, f)),
        Err(_) => Err(PError {
            msg: format!("Invalid float literal: {}", s),
            ctx: src.ctx,
        }),
    }
}

fn boolean(src: Cursor) -> PResult<bool> {
    match parse!(tok(ident), "Bool is not 'true'", src)? {
        (src, "true") => Ok((src, true)),
        (src, "false") => Ok((src, false)),
        (src, ident) => Err(PError {
            msg: format!("[boolean] Expected 'true' or 'false', found {ident}"),
            ctx: src.ctx,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_literal() {
        let src = Cursor::new("not_a_literal");
        assert!(parse_literal(src).is_err());

        let src = Cursor::new("1.0.0");
        let (src, v) = parse_literal(src.clone()).unwrap();
        assert_eq!(v, Literal::Float(1.0));
        assert_eq!(src.remainder, ".0");
        // assert!(parse_literal(src).is_err());

        let src = Cursor::new("truefalse");
        assert!(parse_literal(src).is_err());
    }

    #[test]
    fn literal_bool() {
        let src = Cursor::new("true");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Bool(true));
        assert_eq!(src.remainder, "");

        let src = Cursor::new("false");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Bool(false));
        assert_eq!(src.remainder, "");
    }

    #[test]
    fn literal_int() {
        let src = Cursor::new("123");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Int(123));
        assert_eq!(src.remainder, "");
    }

    #[test]
    fn literal_float() {
        let src = Cursor::new("1.23");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Float(1.23));
        assert_eq!(src.remainder, "");

        // let src = Cursor::new("-0.0123");
        // let (src, v) = parse_literal(src).unwrap();
        // assert_eq!(v, Literal::Float(-0.0123));
        // assert_eq!(src.remainder, "");
    }
}
