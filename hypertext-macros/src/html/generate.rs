use std::{
    iter,
    ops::{Deref, DerefMut},
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    LitStr, braced,
    parse::Parse,
    token::{Brace, Paren},
};

use super::UnquotedName;

pub fn lazy<T: Parse + Generate>(tokens: TokenStream, move_: bool) -> syn::Result<TokenStream> {
    let mut g = Generator::new_closure(T::CONTEXT);

    let len_estimate = tokens.to_string().len();

    g.push(syn::parse2::<T>(tokens)?);

    let block = g.finish();

    let buffer_ident = Generator::buffer_ident();

    let move_token = move_.then(|| quote!(move));

    let marker_ident = T::CONTEXT.marker_type();

    Ok(quote! {
        ::hypertext::Lazy::<_, #marker_ident>::dangerously_create(
            #move_token |#buffer_ident: &mut ::hypertext::Buffer<#marker_ident>| {
                #buffer_ident.dangerously_get_string().reserve(#len_estimate);
                #block
            }
        )
    })
}

pub fn literal<T: Parse + Generate>(tokens: TokenStream) -> syn::Result<TokenStream> {
    let mut g = Generator::new_static(T::CONTEXT);

    g.push(syn::parse2::<T>(tokens)?);

    let literal = g.finish().to_token_stream();

    let marker_ident = T::CONTEXT.marker_type();

    Ok(quote! {
        ::hypertext::Raw::<_, #marker_ident>::dangerously_create(#literal)
    })
}

pub struct Generator {
    lazy: bool,
    context: Context,
    brace_token: Brace,
    parts: Vec<Part>,
    checks: Checks,
}

impl Generator {
    pub fn buffer_ident() -> Ident {
        Ident::new("__hypertext_buffer", Span::mixed_site())
    }

    fn new_closure(context: Context) -> Self {
        Self::new_with_brace(context, true, Brace::default())
    }

    fn new_static(context: Context) -> Self {
        Self::new_with_brace(context, false, Brace::default())
    }

    const fn new_with_brace(context: Context, lazy: bool, brace_token: Brace) -> Self {
        Self {
            lazy,
            context,
            brace_token,
            parts: Vec::new(),
            checks: Checks::new(),
        }
    }

    fn finish(self) -> AnyBlock {
        let mut stmts = self.checks.to_token_stream();

        if self.lazy {
            let buffer_ident = Self::buffer_ident();

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
                            #buffer_ident.dangerously_get_string().push_str(::core::concat!(#(#static_parts),*));
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
        let mut g = Self::new_with_brace(self.context, true, brace_token);

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

    pub fn push_escaped_lit(&mut self, context: Context, lit: &LitStr) {
        let value = lit.value();
        let escaped_value = match context {
            Context::Node => html_escape::encode_text(&value),
            Context::AttributeValue => html_escape::encode_double_quoted_attribute(&value),
        };

        self.parts
            .push(Part::Static(LitStr::new(&escaped_value, lit.span())));
    }

    pub fn push_lits(&mut self, literals: Vec<LitStr>) {
        for lit in literals {
            self.parts.push(Part::Static(lit));
        }
    }

    pub fn push_expr(&mut self, paren_token: Paren, context: Context, expr: impl ToTokens) {
        let buffer_ident = Self::buffer_ident();
        let buffer_expr = match (self.context, context) {
            (Context::Node, Context::Node) | (Context::AttributeValue, Context::AttributeValue) => {
                quote!(#buffer_ident)
            }
            (Context::Node, Context::AttributeValue) => {
                quote!(&mut #buffer_ident.as_attribute_buffer())
            }
            (Context::AttributeValue, Context::Node) => unreachable!(),
        };

        let mut paren_expr = TokenStream::new();
        paren_token.surround(&mut paren_expr, |tokens| expr.to_tokens(tokens));
        let reference = quote_spanned!(paren_token.span=> &);
        self.push_stmt(quote! {
            ::hypertext::Renderable::render_to(
                #reference #paren_expr,
                #buffer_expr
            );
        });
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

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Node,
    AttributeValue,
}

impl Context {
    pub fn marker_type(self) -> TokenStream {
        let ident = match self {
            Self::Node => Ident::new("Node", Span::mixed_site()),
            Self::AttributeValue => Ident::new("AttributeValue", Span::mixed_site()),
        };

        quote!(::hypertext::context::#ident)
    }
}

pub trait Generate {
    const CONTEXT: Context;
    fn generate(&self, g: &mut Generator);
}

impl<T: Generate> Generate for &T {
    const CONTEXT: Context = T::CONTEXT;

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
            const _: fn() = || {
                #[allow(unused_imports)]
                use hypertext_elements::*;

                #[doc(hidden)]
                fn check_element<
                    T: ::hypertext::validation::Element<Kind = K>,
                    K: ::hypertext::validation::ElementKind
                >(_: T) {}

                #(#checks)*
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
                    check_element::<_, #kind>(#el);
                }
            });

        let el = Ident::new_raw(
            &self.ident,
            self.opening_spans
                .first()
                .copied()
                .unwrap_or_else(Span::mixed_site),
        );

        let attr_checks = self
            .attributes
            .iter()
            .map(|attr| attr.to_token_stream_with_el(&el));

        quote! {
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
                    let _: #kind = <#el>::#ident;
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
