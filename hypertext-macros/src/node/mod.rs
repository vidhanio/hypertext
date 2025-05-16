#![expect(clippy::struct_field_names)]

mod control;

use std::{
    fmt::{self, Display, Formatter, Write},
    marker::PhantomData,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token, bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
};

pub use self::control::*;
use crate::generate::{
    AnyBlock, AttributeCheckKind, ElementCheck, ElementKind, Generate, Generator,
};

pub mod kw {
    use syn::LitStr;

    syn::custom_keyword!(data);

    impl data {
        pub fn lit(self) -> LitStr {
            LitStr::new("data", self.span)
        }
    }
}

pub trait Syntax {
    type NodeSeparator: Parse;
}

pub struct Markup<S: Syntax> {
    pub doctype: Option<Span>,
    pub nodes: Nodes<S, ElementNode<S>>,
}

impl<S: Syntax> Generate for Markup<S> {
    fn generate(&self, g: &mut Generator) {
        if let Some(doctype) = self.doctype {
            g.push_spanned_str("<!DOCTYPE html>", doctype);
        }

        g.push(&self.nodes);
    }
}

pub trait Node<S: Syntax>: Generate + Sized {
    type Child: Node<S>;

    fn is_control(&self) -> bool;

    fn as_group(&self) -> Option<&Group<S, Self::Child>>;

    fn in_block(&self, g: &mut Generator) -> AnyBlock {
        if let Some(group) = self.as_group() {
            group.block(g)
        } else {
            g.block_with(Brace::default(), |g| g.push(self))
        }
    }
}

pub enum ElementNode<S: Syntax> {
    Element(Element<S>),
    Component(Component<S>),
    Literal(Literal),
    Control(Control<S, Self>),
    Expr(ParenExpr),
    Group(Group<S, Self>),
}

impl<S: Syntax> Node<S> for ElementNode<S> {
    type Child = Self;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for ElementNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Element(element) => g.push(element),
            Self::Component(component) => g.push(component),
            Self::Literal(lit) => g.push_text_lit(&lit.lit_str()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_text(g),
            Self::Group(group) => g.push(group),
        }
    }
}

pub struct ParenExpr {
    pub paren_token: Paren,
    pub expr: TokenStream,
}

impl ParenExpr {
    fn generate_text(&self, g: &mut Generator) {
        g.push_text_expr(self.paren_token, &self.expr);
    }

    fn generate_attribute(&self, g: &mut Generator) {
        g.push_attribute_expr(self.paren_token, &self.expr);
    }
}

impl Parse for ParenExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            paren_token: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}

pub struct Group<S: Syntax, N: Node<S>>(pub Nodes<S, N>);

impl<S: Syntax, N: Node<S>> Group<S, N> {
    fn block(&self, g: &mut Generator) -> AnyBlock {
        self.0.block(g, Brace::default())
    }
}

impl<S: Syntax, N: Node<S>> Generate for Group<S, N> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.0);
    }
}

pub struct Nodes<S: Syntax, N: Node<S>> {
    pub nodes: Vec<N>,
    pub phantom: PhantomData<S>,
}

impl<S: Syntax, N: Node<S>> Nodes<S, N> {
    fn block(&self, g: &mut Generator, brace_token: Brace) -> AnyBlock {
        g.block_with(brace_token, |g| {
            g.push_all(&self.nodes);
        })
    }
}

impl<S: Syntax, N: Node<S> + Parse> Parse for Nodes<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: input
                .call(Punctuated::<N, S::NodeSeparator>::parse_terminated)?
                .into_iter()
                .collect(),
            phantom: PhantomData,
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for Nodes<S, N> {
    fn generate(&self, g: &mut Generator) {
        if self.nodes.iter().any(Node::is_control) {
            g.push_in_block(Brace::default(), |g| g.push_all(&self.nodes));
        } else {
            g.push_all(&self.nodes);
        }
    }
}

pub struct Element<S: Syntax> {
    pub name: UnquotedName,
    pub attrs: Vec<Attribute<S>>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Element<S> {
    fn generate(&self, g: &mut Generator) {
        let mut el_checks = ElementCheck::new(&self.name, self.body.kind());

        g.push_str("<");
        g.push_lits(self.name.lits());

        for attr in &self.attrs {
            g.push(attr);
            if let Some((kind, ident, spans)) = attr.name.check() {
                el_checks.push_attribute(kind, ident, spans);
            }
        }

        g.push_str(">");

        match &self.body {
            ElementBody::Normal {
                children,
                closing_name,
            } => {
                let name = closing_name.as_ref().map_or(&self.name, |closing_name| {
                    el_checks.set_closing_name(closing_name);
                    closing_name
                });

                g.push(children);
                g.push_str("</");
                g.push_lits(name.lits());
                g.push_str(">");
            }
            ElementBody::Void => {}
        }

        g.record_element(el_checks);
    }
}

pub enum ElementBody<S: Syntax> {
    Normal {
        children: Nodes<S, ElementNode<S>>,
        closing_name: Option<UnquotedName>,
    },
    Void,
}

impl<S: Syntax> ElementBody<S> {
    const fn kind(&self) -> ElementKind {
        match self {
            Self::Normal { .. } => ElementKind::Normal,
            Self::Void => ElementKind::Void,
        }
    }
}

pub struct Attribute<S: Syntax> {
    pub name: AttributeName,
    pub kind: AttributeKind<S>,
}

impl<S: Syntax> Generate for Attribute<S> {
    fn generate(&self, g: &mut Generator) {
        match &self.kind {
            AttributeKind::Value { value, toggle, .. } => {
                if let Some(toggle) = toggle {
                    g.push_conditional(toggle.parenthesized(), |g| {
                        g.push_str(" ");
                        g.push_lits(self.name.lits());
                        g.push_str("=\"");
                        g.push(value);
                        g.push_str("\"");
                    });
                } else {
                    g.push_str(" ");
                    g.push_lits(self.name.lits());
                    g.push_str("=\"");
                    g.push(value);
                    g.push_str("\"");
                }
            }
            AttributeKind::Option(option) => {
                let option_expr = &option.expr;

                let value = Ident::new("value", Span::mixed_site());

                g.push_conditional(
                    quote!(let ::core::option::Option::Some(#value) = (#option_expr)),
                    |g| {
                        g.push_str(" ");
                        g.push_lits(self.name.lits());
                        g.push_str("=\"");
                        g.push_attribute_expr(Paren::default(), &value);
                        g.push_str("\"");
                    },
                );
            }
            AttributeKind::Empty(Some(toggle)) => {
                g.push_conditional(toggle.parenthesized(), |g| {
                    g.push_str(" ");
                    g.push_lits(self.name.lits());
                });
            }
            AttributeKind::Empty(None) => {
                g.push_str(" ");
                g.push_lits(self.name.lits());
            }
            AttributeKind::ClassList(classes) => {
                g.push_str(" ");
                g.push_lits(self.name.lits());
                g.push_str("=\"");

                for (i, class) in classes.iter().enumerate() {
                    if let Some(toggle) = &class.toggle {
                        g.push_conditional(toggle.parenthesized(), |g| {
                            if i > 0 {
                                g.push_str(" ");
                            }

                            g.push(&class.value);
                        });
                    } else {
                        if i > 0 {
                            g.push_str(" ");
                        }

                        g.push(&class.value);
                    }
                }

                g.push_str("\"");
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum AttributeName {
    Data {
        data_token: kw::data,
        hyphen: Token![-],
        rest: UnquotedName,
    },
    Namespace {
        namespace: UnquotedName,
        colon: Token![:],
        rest: UnquotedName,
    },
    Symbol {
        symbol: AttributeSymbol,
        rest: UnquotedName,
    },
    Normal(UnquotedName),
}

impl AttributeName {
    pub fn check(&self) -> Option<(AttributeCheckKind, String, Vec<Span>)> {
        match self {
            Self::Data { .. } => None,
            Self::Namespace {
                namespace, colon, ..
            } => {
                let mut spans = namespace.spans();
                spans.push(colon.span());
                Some((
                    AttributeCheckKind::Namespace,
                    namespace.ident_string(),
                    spans,
                ))
            }
            Self::Symbol { symbol, .. } => Some((
                AttributeCheckKind::Symbol,
                symbol.ident_string(),
                vec![symbol.span()],
            )),
            Self::Normal(name) => Some((
                AttributeCheckKind::Normal,
                name.ident_string(),
                name.spans(),
            )),
        }
    }

    fn lits(&self) -> Vec<LitStr> {
        match self {
            Self::Data {
                data_token,
                hyphen,
                rest,
            } => {
                let mut lits = vec![data_token.lit(), LitStr::new("-", hyphen.span)];

                lits.append(&mut rest.lits());

                lits
            }
            Self::Namespace {
                namespace,
                colon,
                rest,
            } => {
                let mut lits = namespace.lits();
                lits.push(LitStr::new(":", colon.span));
                lits.append(&mut rest.lits());
                lits
            }
            Self::Symbol { symbol, rest } => {
                let mut lits = vec![symbol.lit()];
                lits.append(&mut rest.lits());
                lits
            }
            Self::Normal(unquoted_name) => unquoted_name.lits(),
        }
    }
}

impl Parse for AttributeName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::data) && input.peek2(Token![-]) {
            Ok(Self::Data {
                data_token: input.parse()?,
                hyphen: input.parse()?,
                rest: input.call(UnquotedName::parse_any)?,
            })
        } else if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            let name = input.parse()?;
            if input.peek(Token![:]) {
                Ok(Self::Namespace {
                    namespace: name,
                    colon: input.parse()?,
                    rest: input.call(UnquotedName::parse_any)?,
                })
            } else {
                Ok(Self::Normal(name))
            }
        } else if lookahead.peek(Token![@]) || lookahead.peek(Token![:]) {
            Ok(Self::Symbol {
                symbol: input.parse()?,
                rest: input.call(UnquotedName::parse_any)?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum AttributeSymbol {
    At(Token![@]),
    Colon(Token![:]),
}

impl AttributeSymbol {
    fn lit(&self) -> LitStr {
        match self {
            Self::At(token) => LitStr::new("@", token.span()),
            Self::Colon(token) => LitStr::new(":", token.span()),
        }
    }

    fn ident_string(&self) -> String {
        match self {
            Self::At(_) => "at".to_string(),
            Self::Colon(_) => "colon".to_string(),
        }
    }

    fn span(&self) -> Span {
        match self {
            Self::At(token) => token.span(),
            Self::Colon(token) => token.span(),
        }
    }
}

impl Parse for AttributeSymbol {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![@]) {
            input.parse().map(Self::At)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum AttributeKind<S: Syntax> {
    Value {
        value: UnquotedValueNode<S>,
        toggle: Option<Toggle>,
    },
    Empty(Option<Toggle>),
    Option(Toggle),
    ClassList(Vec<Class<S>>),
}

pub enum QuotedValueNode<S: Syntax> {
    Literal(Literal),
    Group(Group<S, Self>),
    Control(Control<S, Self>),
    Expr(ParenExpr),
}

impl<S: Syntax> Node<S> for QuotedValueNode<S> {
    type Child = Self;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for QuotedValueNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Literal(lit) => g.push_attribute_lit(&lit.lit_str()),
            Self::Group(block) => g.push(block),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_attribute(g),
        }
    }
}

pub struct Class<S: Syntax> {
    pub value: UnquotedValueNode<S>,
    pub toggle: Option<Toggle>,
}

pub enum UnquotedValueNode<S: Syntax> {
    UnquotedName(UnquotedName),
    Str(LitStr),
    Group(Group<S, QuotedValueNode<S>>),
    Control(Control<S, QuotedValueNode<S>>),
    Expr(ParenExpr),
}

impl<S: Syntax> Node<S> for UnquotedValueNode<S> {
    type Child = QuotedValueNode<S>;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for UnquotedValueNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::UnquotedName(name) => g.push_lits(name.lits()),
            Self::Group(block) => g.push(block),
            Self::Str(lit) => g.push_attribute_lit(&lit.clone()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_attribute(g),
        }
    }
}

pub struct Toggle {
    pub bracket_token: Bracket,
    pub expr: TokenStream,
}

impl Toggle {
    fn parenthesized(&self) -> TokenStream {
        let paren_token = Paren {
            span: self.bracket_token.span,
        };

        let mut tokens = TokenStream::new();

        paren_token.surround(&mut tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });

        tokens
    }

    pub fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
        if input.peek(Bracket) {
            input.parse().map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Parse for Toggle {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            bracket_token: bracketed!(content in input),
            expr: content.parse()?,
        })
    }
}

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    fn generate(&self, g: &mut Generator) {
        let fields = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            let value = &attr.value.expr();

            quote!(#name: #value)
        });

        let children = match &self.body {
            ElementBody::Normal { children, .. } => {
                let output_ident = Ident::new("hypertext_output", Span::mixed_site());

                let block = g.block_with(Brace::default(), |g| {
                    g.push(children);
                });

                let lazy = quote! {
                    ::hypertext::Lazy({
                        extern crate alloc;

                        move |#output_ident: &mut alloc::string::String|
                            #block
                    })
                };

                quote!(
                    children: #lazy,
                )
            }
            ElementBody::Void => quote!(),
        };

        let name = &self.name;

        let init = quote! {
            #name {
                #(#fields,)*
                #children
            }
        };

        g.push_text_expr(Paren::default(), &init);
    }
}

pub struct ComponentAttribute {
    pub name: Ident,
    pub value: ComponentAttributeValue,
}

impl Parse for ComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            value: {
                input.parse::<Token![=]>()?;

                input.parse()?
            },
        })
    }
}

pub enum ComponentAttributeValue {
    UnquotedName(UnquotedName),
    Literal(Literal),
    Expr(ParenExpr),
}

impl ComponentAttributeValue {
    fn expr(&self) -> TokenStream {
        match self {
            Self::UnquotedName(name) => {
                let strs = name.lits();

                quote!(::core::concat!(#(#strs),*))
            }
            Self::Literal(lit) => match lit {
                Literal::Str(lit) => lit.to_token_stream(),
                Literal::Int(lit) => lit.to_token_stream(),
                Literal::Bool(lit) => lit.to_token_stream(),
                Literal::Float(lit) => lit.to_token_stream(),
            },
            Self::Expr(expr) => {
                let mut tokens = TokenStream::new();

                expr.paren_token.surround(&mut tokens, |tokens| {
                    expr.expr.to_tokens(tokens);
                });

                tokens
            }
        }
    }
}

impl Parse for ComponentAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::UnquotedName)
        } else if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct UnquotedName(pub Vec<NameFragment>);

impl UnquotedName {
    pub fn ident_string(&self) -> String {
        let mut s = String::new();

        for fragment in &self.0 {
            match fragment {
                NameFragment::Ident(ident) => {
                    _ = write!(s, "{ident}");
                }
                NameFragment::Number(num) => {
                    _ = write!(s, "{num}");
                }
                NameFragment::Hyphen(_) => {
                    s.push('_');
                }
                NameFragment::Colon(_) => {
                    unreachable!("colons should never be in idents")
                }
            }
        }

        if s == "super"
            || s == "self"
            || s == "Self"
            || s == "extern"
            || s == "crate"
            || s == "_"
            || s.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            s.insert(0, '_');
        }

        s
    }

    pub fn is_component(&self) -> bool {
        matches!(
            self.0.as_slice(),
            [NameFragment::Ident(ident)]
                if ident.to_string().chars().next().is_some_and(|c| c.is_ascii_uppercase())
        )
    }

    pub fn spans(&self) -> Vec<Span> {
        let mut spans = Vec::new();

        for fragment in &self.0 {
            spans.push(fragment.span());
        }

        spans
    }

    pub fn lits(&self) -> Vec<LitStr> {
        let mut strs = Vec::new();

        for fragment in &self.0 {
            strs.push(LitStr::new(&fragment.to_string(), fragment.span()));
        }

        strs
    }

    fn parse_any(input: ParseStream) -> syn::Result<Self> {
        let mut name = Vec::new();

        while input.peek(Token![-])
            || input.peek(Token![:])
            || (name.last().is_none_or(NameFragment::is_punct)
                && (input.peek(Ident::peek_any) || input.peek(LitInt)))
        {
            name.push(input.parse()?);
        }

        Ok(Self(name))
    }
}

impl Parse for UnquotedName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let mut name = Vec::new();

        if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            name.push(input.parse()?);

            while input.peek(Token![-])
                || (name.last().is_none_or(NameFragment::is_punct)
                    && (input.peek(Ident::peek_any) || input.peek(LitInt)))
            {
                name.push(input.parse()?);
            }

            Ok(Self(name))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum NameFragment {
    Ident(Ident),
    Number(LitInt),
    Hyphen(Token![-]),
    Colon(Token![:]),
}

impl NameFragment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Number(num) => num.span(),
            Self::Hyphen(hyphen) => hyphen.span(),
            Self::Colon(colon) => colon.span(),
        }
    }

    const fn is_punct(&self) -> bool {
        matches!(self, Self::Hyphen(_) | Self::Colon(_))
    }
}

impl Parse for NameFragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Ident)
        } else if lookahead.peek(LitInt) {
            let int = input.parse::<LitInt>()?;

            if !int.suffix().is_empty() {
                return Err(syn::Error::new_spanned(
                    &int,
                    "integer suffixes are not allowed in names",
                ));
            }

            Ok(Self::Number(int))
        } else if lookahead.peek(Token![-]) {
            input.parse().map(Self::Hyphen)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Display for NameFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "{ident}"),
            Self::Number(num) => write!(f, "{num}"),
            Self::Hyphen(_) => f.write_str("-"),
            Self::Colon(_) => f.write_str(":"),
        }
    }
}

pub enum Literal {
    Str(LitStr),
    Int(LitInt),
    Bool(LitBool),
    Float(LitFloat),
}

impl Literal {
    fn lit_str(&self) -> LitStr {
        match self {
            Self::Str(lit) => lit.clone(),
            Self::Int(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Bool(lit) => LitStr::new(&lit.value.to_string(), lit.span()),
            Self::Float(lit) => LitStr::new(&lit.to_string(), lit.span()),
        }
    }
}

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            input.parse().map(Self::Str)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else {
            Err(lookahead.error())
        }
    }
}
