use std::path::Path;

use rustc_ast::{
    visit::{walk_crate, walk_expr, Visitor},
    Expr, ExprKind,
};
use rustc_session::parse::ParseSess;
use rustc_span::Span;

#[derive(Default)]
struct Collector {
    functions: Vec<Span>,
}

impl<'ast> Visitor<'ast> for Collector {
    fn visit_expr(&mut self, e: &'ast Expr) {
        if let ExprKind::Call(f, _) = &e.kind {
            self.functions.push(f.span);
        }
        walk_expr(self, e);
    }
}

pub fn collect(file: &Path) -> Vec<String> {
    rustc_span::create_default_session_globals_then(|| {
        let sess = ParseSess::with_silent_emitter(None);
        let span_to_string = |span: Span| sess.source_map().span_to_snippet(span).unwrap();
        let mut collector = Collector::default();
        if let Ok(krate) = rustc_parse::parse_crate_from_file(file, &sess) {
            walk_crate(&mut collector, &krate);
        }
        collector.functions.drain(..).map(span_to_string).collect()
    })
}
