use crate::parse;

use super::*;

pub fn parse_literal(src: Cursor) -> PResult<Literal> {
    if let Ok((src, int)) = int(src.clone()) {
        Ok((src, Literal::Int(int)))
    } else if let Ok((src, float)) = decimal(src.clone()) {
        Ok((src, Literal::Float(float)))
    } else if let Ok((src, bool)) = boolean(src.clone()) {
        Ok((src, Literal::Bool(bool)))
    } else {
        Err(PError {
            msg: format!(
                "[parse_literal] Could not parse literal: {}",
                &src.remainder[..30]
            ),
            ctx: src.ctx,
        })
    }
}

fn boolean(src: Cursor) -> PResult<bool> {
    match parse!(tok(ident), "Bool is not 'true'", src)? {
        (src, "true") => Ok((src, true)),
        (src, "false") => Ok((src, true)),
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
        assert!(parse_literal(src).is_err());

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

        let src = Cursor::new("-123");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Int(-123));
        assert_eq!(src.remainder, "");
    }

    #[test]
    fn literal_float() {
        let src = Cursor::new("1.23");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Float(1.23));
        assert_eq!(src.remainder, "");

        let src = Cursor::new("-0.0123");
        let (src, v) = parse_literal(src).unwrap();
        assert_eq!(v, Literal::Float(-0.0123));
        assert_eq!(src.remainder, "");
    }
}
