use std::{
    iter,
    ops::{Deref, DerefMut},
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    LitStr, Token, braced,
    parse::Parse,
    token::{Brace, Paren},
};

use super::UnquotedName;

pub fn closure<T: Parse + Generate>(tokens: TokenStream) -> syn::Result<TokenStream> {
    let mut g = Generator::new_closure();

    let len_estimate = tokens.to_string().len();

    g.push(syn::parse2::<T>(tokens)?);

    let block = g.finish();

    let output_ident = Generator::output_ident();

    Ok(quote! {
        |#output_ident: &mut ::hypertext::proc_macros::String| {
            #output_ident.reserve(#len_estimate);
            #block
        }
    })
}

pub fn literal<T: Parse + Generate>(tokens: TokenStream) -> syn::Result<TokenStream> {
    let mut g = Generator::new_static();

    g.push(syn::parse2::<T>(tokens)?);

    Ok(g.finish().to_token_stream())
}

pub struct Generator {
    output_ident: Option<Ident>,
    brace_token: Brace,
    parts: Vec<Part>,
    checks: Checks,
}

impl Generator {
    pub fn output_ident() -> Ident {
        Ident::new("hypertext_output", Span::mixed_site())
    }

    fn new_closure() -> Self {
        Self::new_with_brace(Some(Self::output_ident()), Brace::default())
    }

    fn new_static() -> Self {
        Self::new_with_brace(None, Brace::default())
    }

    const fn new_with_brace(output_ident: Option<Ident>, brace_token: Brace) -> Self {
        Self {
            output_ident,
            brace_token,
            parts: Vec::new(),
            checks: Checks::new(),
        }
    }

    fn finish(self) -> AnyBlock {
        let mut stmts = self.checks.to_token_stream();

        if let Some(output_ident) = self.output_ident {
            let mut parts = self.parts.into_iter();

            while let Some(part) = parts.next() {
                match part {
                    Part::Static(lit) => {
                        let mut dynamic_stmt = None;
                        let static_parts =
                            iter::once(lit).chain(parts.by_ref().map_while(|part| match part {
                                Part::Static(lit) => Some(lit),
                                Part::Dynamic(stmt) => {
                                    dynamic_stmt = Some(stmt);
                                    None
                                }
                            }));

                        stmts.extend(quote! {
                            #output_ident.push_str(::core::concat!(#(#static_parts),*));
                        });
                        stmts.extend(dynamic_stmt);
                    }
                    Part::Dynamic(stmt) => {
                        stmts.extend(stmt);
                    }
                }
            }
        } else {
            let mut static_parts = Vec::new();

            for part in self.parts {
                match part {
                    Part::Static(lit) => static_parts.push(lit),
                    Part::Dynamic(stmt) => stmts.extend(
                        syn::Error::new_spanned(
                            stmt,
                            "static evaluation cannot contain dynamic parts",
                        )
                        .to_compile_error(),
                    ),
                }
            }

            stmts.extend(quote!(::core::concat!(#(#static_parts),*)));
        }

        AnyBlock {
            brace_token: self.brace_token,
            stmts,
        }
    }

    pub fn block_with(&mut self, brace_token: Brace, f: impl FnOnce(&mut Self)) -> AnyBlock {
        let mut g = Self::new_with_brace(self.output_ident.clone(), brace_token);

        f(&mut g);

        self.checks.append(&mut g.checks);

        g.finish()
    }

    pub fn push_in_block(&mut self, brace_token: Brace, f: impl FnOnce(&mut Self)) {
        let block = self.block_with(brace_token, f);
        self.push_stmt(block);
    }

    pub fn push_str(&mut self, s: &'static str) {
        self.push_spanned_str(s, Span::mixed_site());
    }

    pub fn push_spanned_str(&mut self, s: &'static str, span: Span) {
        self.parts.push(Part::Static(LitStr::new(s, span)));
    }

    pub fn push_text_lit(&mut self, lit: &LitStr) {
        let value = lit.value();
        let escaped_value = html_escape::encode_text(&value);

        self.parts
            .push(Part::Static(LitStr::new(&escaped_value, lit.span())));
    }

    pub fn push_attribute_lit(&mut self, lit: &LitStr) {
        let value = lit.value();
        let escaped_value = html_escape::encode_double_quoted_attribute(&value);

        self.parts
            .push(Part::Static(LitStr::new(&escaped_value, lit.span())));
    }

    pub fn push_lits(&mut self, literals: Vec<LitStr>) {
        for lit in literals {
            self.parts.push(Part::Static(lit));
        }
    }

    pub fn push_text_expr(&mut self, paren_token: Paren, expr: impl ToTokens) {
        let output_ident = &self.output_ident;
        let mut paren_expr = TokenStream::new();
        paren_token.surround(&mut paren_expr, |tokens| expr.to_tokens(tokens));
        let reference = Token![&](paren_token.span.join());
        self.push_stmt(
            quote!(::hypertext::Renderable::render_to(#reference #paren_expr, #output_ident);),
        );
    }

    pub fn push_attribute_expr(&mut self, paren_token: Paren, expr: impl ToTokens) {
        let output_ident = &self.output_ident;
        let mut paren_expr = TokenStream::new();
        paren_token.surround(&mut paren_expr, |tokens| expr.to_tokens(tokens));
        let reference = quote_spanned!(paren_token.span=> &);
        self.push_stmt(
            quote!(::hypertext::AttributeRenderable::render_attribute_to(#reference #paren_expr, #output_ident);),
        );
    }

    pub fn push_stmt(&mut self, stmt: impl ToTokens) {
        self.parts.push(Part::Dynamic(stmt.to_token_stream()));
    }

    pub fn push_conditional(&mut self, cond: impl ToTokens, f: impl FnOnce(&mut Self)) {
        let then_block = self.block_with(Brace::default(), f);
        self.push_stmt(quote! {
            if #cond #then_block
        });
    }

    pub fn push(&mut self, value: impl Generate) {
        value.generate(self);
    }

    pub fn record_element(&mut self, el_checks: ElementCheck) {
        self.checks.push(el_checks);
    }

    pub fn push_all(&mut self, values: impl IntoIterator<Item = impl Generate>) {
        for value in values {
            self.push(value);
        }
    }
}

enum Part {
    Static(LitStr),
    Dynamic(TokenStream),
}

pub trait Generate {
    fn generate(&self, g: &mut Generator);
}

impl<T: Generate> Generate for &T {
    fn generate(&self, g: &mut Generator) {
        (*self).generate(g);
    }
}

struct Checks {
    elements: Vec<ElementCheck>,
}

impl Checks {
    const fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn append(&mut self, other: &mut Self) {
        self.elements.append(&mut other.elements);
    }
}

impl ToTokens for Checks {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_empty() {
            return;
        }

        let checks = &self.elements;

        quote! {
            const _: () = {
                #[allow(unused_imports)]
                use html_elements::*;

                const _: () = {
                    #[doc(hidden)]
                    const fn check_element<
                        T: ::hypertext::validation::Element<Kind = K>,
                        K: ::hypertext::validation::ElementKind
                    >() {}

                    #(#checks)*
                };
            };
        }
        .to_tokens(tokens);
    }
}

impl Deref for Checks {
    type Target = Vec<ElementCheck>;

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl DerefMut for Checks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

pub struct ElementCheck {
    ident: String,
    kind: ElementKind,
    opening_spans: Vec<Span>,
    closing_spans: Vec<Span>,
    attributes: Vec<AttributeCheck>,
}

impl ElementCheck {
    pub fn new(el_name: &UnquotedName, element_kind: ElementKind) -> Self {
        Self {
            ident: el_name.ident_string(),
            kind: element_kind,
            opening_spans: el_name.spans(),
            closing_spans: Vec::new(),
            attributes: Vec::new(),
        }
    }

    pub fn set_closing_spans(&mut self, spans: Vec<Span>) {
        self.closing_spans = spans;
    }

    pub fn push_attribute(&mut self, attr: AttributeCheck) {
        self.attributes.push(attr);
    }
}

impl ToTokens for ElementCheck {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let kind = self.kind;

        let el_checks = self
            .opening_spans
            .iter()
            .chain(&self.closing_spans)
            .map(|span| {
                let el = Ident::new_raw(&self.ident, *span);

                quote! {
                    let _: #el = #el;
                }
            });

        let el = Ident::new_raw(
            &self.ident,
            self.opening_spans
                .first()
                .copied()
                .unwrap_or_else(Span::mixed_site),
        );

        let check_kind = quote! {
            check_element::<#el, #kind>();
        };

        let attr_checks = self
            .attributes
            .iter()
            .map(|attr| attr.to_token_stream_with_el(&el));

        quote! {
            #check_kind
            #(#el_checks)*
            #(#attr_checks)*
        }
        .to_tokens(tokens);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ElementKind {
    Normal,
    Void,
}

impl ToTokens for ElementKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Normal => quote!(::hypertext::validation::Normal),
            Self::Void => quote!(::hypertext::validation::Void),
        }
        .to_tokens(tokens);
    }
}

pub struct AttributeCheck {
    kind: AttributeCheckKind,
    ident: String,
    spans: Vec<Span>,
}

impl AttributeCheck {
    pub const fn new(kind: AttributeCheckKind, ident: String, spans: Vec<Span>) -> Self {
        Self { kind, ident, spans }
    }

    fn to_token_stream_with_el(&self, el: &Ident) -> TokenStream {
        let kind = &self.kind;

        self.spans
            .iter()
            .map(|span| {
                let ident = Ident::new_raw(&self.ident, *span);

                quote! {
                    let _: #kind = #el::#ident;
                }
            })
            .collect()
    }
}

pub enum AttributeCheckKind {
    Normal,
    Namespace,
    Symbol,
}

impl ToTokens for AttributeCheckKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Normal => quote!(::hypertext::validation::Attribute),
            Self::Namespace => quote!(::hypertext::validation::AttributeNamespace),
            Self::Symbol => quote!(::hypertext::validation::AttributeSymbol),
        }
        .to_tokens(tokens);
    }
}

pub struct AnyBlock {
    pub brace_token: Brace,
    pub stmts: TokenStream,
}

impl Parse for AnyBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            brace_token: braced!(content in input),
            stmts: content.parse()?,
        })
    }
}

impl ToTokens for AnyBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            self.stmts.to_tokens(tokens);
        });
    }
}
