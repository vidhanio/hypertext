use std::iter;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_quote, spanned::Spanned, token::Brace, Block, Expr, ExprBlock, ExprIf, LitStr, Stmt,
    Token,
};

#[allow(clippy::needless_pass_by_value)]
pub fn normal(len_estimate: usize, output_ident: Ident, value: impl Generate) -> TokenStream {
    let mut gen = Generator::new(output_ident.clone());

    gen.push(value);

    let block = gen.finish();

    quote! {
        ::hypertext::Renderable(move |#output_ident| {
            #output_ident.reserve(#len_estimate);
            #block
        })
    }
}

pub fn r#static(output_ident: Ident, value: impl Generate) -> TokenStream {
    let mut gen = Generator::new(output_ident);

    gen.push(value);

    let block = gen.finish_static();

    quote!(::hypertext::Rendered(#block))
}

pub struct Generator {
    output_ident: Ident,
    parts: Vec<Part>,
    elements: Vec<Ident>,
    attributes: Vec<(Ident, Ident)>,
    void_elements: Vec<Ident>,
}

impl Generator {
    fn new(output_ident: Ident) -> Self {
        Self {
            output_ident,
            parts: Vec::new(),
            elements: Vec::new(),
            attributes: Vec::new(),
            void_elements: Vec::new(),
        }
    }

    fn checks(&self) -> Stmt {
        let elements = self.elements.iter().map(|el| quote!(html_elements::#el;));
        let attributes = self
            .attributes
            .iter()
            .map(|(el, attr)| quote!(let _: ::hypertext::Attribute = html_elements::#el::#attr;));
        let void_elements = self.void_elements.iter().map(|el| {
            quote_spanned! {el.span()=>
                {
                    struct _VoidCheck where html_elements::#el: ::hypertext::VoidElement;
                }
            }
        });

        parse_quote! {
            const _: () = {
                #(#elements)*
                #(#attributes)*
                #(#void_elements)*
            };
        }
    }

    fn finish(self) -> Block {
        let mut stmts = vec![self.checks()];

        let output_ident = self.output_ident;
        let mut parts = self.parts.into_iter();

        while let Some(part) = parts.next() {
            match part {
                Part::Static(lit) => {
                    let mut dynamic_stmt = None;
                    let static_parts =
                        iter::once(lit).chain(parts.by_ref().map_while(|part| match part {
                            Part::Static(lit) => Some(lit),
                            Part::Dynamic(stmt, _) => {
                                dynamic_stmt = Some(stmt);
                                None
                            }
                        }));

                    stmts.push(parse_quote! {
                        #output_ident.push_str(::core::concat!(#(#static_parts),*));
                    });
                    stmts.extend(dynamic_stmt);
                }
                Part::Dynamic(stmt, _) => stmts.push(stmt),
            }
        }

        Block {
            brace_token: Brace::default(),
            stmts,
        }
    }

    fn finish_static(self) -> Block {
        let mut stmts = vec![self.checks()];
        let mut static_parts = Vec::new();

        for part in self.parts {
            match part {
                Part::Static(lit) => static_parts.push(lit),
                Part::Dynamic(_, span) => stmts.push(
                    syn::parse2(
                        syn::Error::new_spanned(
                            Ident::new("_", span.unwrap_or_else(Span::call_site)),
                            "static evaluation cannot contain dynamic parts",
                        )
                        .into_compile_error(),
                    )
                    .unwrap(),
                ),
            }
        }

        stmts.push(Stmt::Expr(
            parse_quote!(::core::concat!(#(#static_parts),*)),
            None,
        ));

        Block {
            brace_token: Brace::default(),
            stmts,
        }
    }

    pub fn block_with(&self, f: impl FnOnce(&mut Self)) -> Block {
        let mut gen = Self::new(self.output_ident.clone());

        f(&mut gen);

        gen.finish()
    }

    pub fn block(&self, value: impl Generate) -> Block {
        self.block_with(|gen| value.generate(gen))
    }

    pub fn in_block(&mut self, f: impl FnOnce(&mut Self)) {
        let mut gen = Self::new(self.output_ident.clone());

        f(&mut gen);

        self.push_expr(ExprBlock {
            attrs: Vec::new(),
            label: None,
            block: gen.finish(),
        });
    }

    pub fn push_str(&mut self, s: &'static str) {
        self.push_spanned_str(s, Span::call_site());
    }

    pub fn push_spanned_str(&mut self, s: &'static str, span: Span) {
        self.parts.push(Part::Static(LitStr::new(s, span)));
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn push_escaped_lit(&mut self, lit: LitStr) {
        let value = lit.value();
        let escaped_value = html_escape::encode_double_quoted_attribute(&value);

        self.parts
            .push(Part::Static(LitStr::new(&escaped_value, lit.span())));
    }

    pub fn push_dynamic(&mut self, stmt: Stmt, span: Option<Span>) {
        self.parts.push(Part::Dynamic(stmt, span));
    }

    pub fn push_conditional(&mut self, cond: &Expr, f: impl FnOnce(&mut Self)) {
        self.push_unspanned_expr(ExprIf {
            attrs: Vec::new(),
            if_token: <Token![if]>::default(),
            cond: Box::new(cond.clone()),
            then_branch: self.block_with(f),
            else_branch: None,
        });
    }

    pub fn push_expr(&mut self, expr: impl Into<Expr> + Spanned) {
        let span = expr.span();
        let expr = expr.into();
        self.push_dynamic(Stmt::Expr(expr, None), Some(span));
    }

    pub fn push_unspanned_expr(&mut self, expr: impl Into<Expr>) {
        self.push_dynamic(Stmt::Expr(expr.into(), None), None);
    }

    pub fn push_rendered_expr(&mut self, expr: &Expr) {
        let output_ident = &self.output_ident;
        self.push_dynamic(
            parse_quote!(::hypertext::Render::render_to(#expr, #output_ident);),
            Some(expr.span()),
        );
    }

    pub fn push_all(&mut self, values: impl IntoIterator<Item = impl Generate>) {
        for value in values {
            self.push(value);
        }
    }

    pub fn push(&mut self, value: impl Generate) {
        value.generate(self);
    }

    pub fn record_void_element(&mut self, el_name: &Ident) {
        self.void_elements.push(el_name.clone());
    }

    pub fn record_element(&mut self, el_name: &Ident) {
        self.elements.push(el_name.clone());
    }

    pub fn record_attribute(&mut self, el_name: &Ident, attr_name: &Ident) {
        self.attributes.push((el_name.clone(), attr_name.clone()));
    }
}

enum Part {
    Static(LitStr),
    Dynamic(Stmt, Option<Span>),
}

pub trait Generate {
    fn generate(&self, gen: &mut Generator);
}

impl<T: Generate> Generate for &T {
    fn generate(&self, gen: &mut Generator) {
        (*self).generate(gen);
    }
}
