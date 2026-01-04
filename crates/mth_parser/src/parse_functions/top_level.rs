use super::*;

pub fn parse_top_level(src: Cursor) -> PResult<TopLevel> {
    pmatch! {src; err = "[parse_top_level]";
        terminated(parse_type_decl, chr(';')), x => TopLevel::TypeDecl(x);
        terminated(parse_mapping, chr(';')), x => TopLevel::MapImpl(x);
        terminated(parse_expr, chr(';')), x => TopLevel::Expr(x);
    }
}
