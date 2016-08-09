// Taken from Rust source code: src/libsyntax/ext/concat.rs
use std::io::Write;
use std::rc::Rc;
use syntax::ast;
use syntax::tokenstream;
use syntax::codemap;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

pub fn expand_syntax_ext(cx: &mut base::ExtCtxt,
                         sp: codemap::Span,
                         tts: &[tokenstream::TokenTree])
                         -> Box<base::MacResult+'static> {
    let es = match base::get_exprs_from_tts(cx, sp, tts) {
        Some(e) => e,
        None => return base::DummyResult::expr(sp)
    };
    let mut accumulator = Vec::new();
    for e in es {
        match e.node {
            ast::ExprKind::Lit(ref lit) => {
                match lit.node {
                    ast::LitKind::Str(ref s, _) |
                    ast::LitKind::Float(ref s, _) |
                    ast::LitKind::FloatUnsuffixed(ref s) => {
                        write!(accumulator, "{}", s).unwrap();
                    }
                    ast::LitKind::Char(c) => {
                        write!(accumulator, "{}", c).unwrap();
                    }
                    ast::LitKind::Int(i, ast::LitIntType::Unsigned(_)) |
                    ast::LitKind::Int(i, ast::LitIntType::Signed(_)) |
                    ast::LitKind::Int(i, ast::LitIntType::Unsuffixed) => {
                        write!(accumulator, "{}", i).unwrap();
                    }
                    ast::LitKind::Bool(b) => {
                        write!(accumulator, "{}", b).unwrap();
                    }
                    ast::LitKind::Byte(b) => {
                        accumulator.push(b);
                    }
                    ast::LitKind::ByteStr(ref bytes) => {
                        accumulator.extend_from_slice(bytes);
                    }
                }
            }
            _ => {
                cx.span_err(e.span, "expected a literal");
            }
        }
    }
    base::MacEager::expr(cx.expr_lit(
        sp,
        ast::LitKind::ByteStr(Rc::new(accumulator))
    ))
}
