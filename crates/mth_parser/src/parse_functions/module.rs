use super::*;

pub fn parse_module(src: Cursor) -> PResult<Module> {
    let mut src = src;
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

    Ok((
        src,
        Module {
            name: None,
            top_level: exprs,
        },
    ))
}
