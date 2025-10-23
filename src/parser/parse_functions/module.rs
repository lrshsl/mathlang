use super::*;

pub fn parse_module<'s>(src: &'s str) -> Module<'s> {
    let mut src = Cursor::new(src);
    let mut exprs = Vec::new();

    loop {
        match parse_top_level(src.clone()) {
            Ok((new_src, tl)) => {
                src = new_src;
                exprs.push(tl);
            }
            Err(_) => break,
        }
    }

    Module {
        name: None,
        top_level: exprs,
    }
}
