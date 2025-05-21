#![expect(clippy::struct_field_names, clippy::large_enum_variant)]

mod control;

use std::{
    fmt::{self, Display, Formatter, Write},
    marker::PhantomData,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token, bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
};

pub use self::control::*;
use crate::generate::{
    AnyBlock, AttributeCheck, AttributeCheckKind, ElementCheck, ElementKind, Generate, Generator,
};

pub mod kw {
    use syn::LitStr;

    syn::custom_keyword!(data);

    impl data {
        pub fn lit(self) -> LitStr {
            LitStr::new("data", self.span)
        }
    }

    syn::custom_keyword!(DOCTYPE);

    impl DOCTYPE {
        pub fn lit(self) -> LitStr {
            LitStr::new("DOCTYPE", self.span)
        }
    }

    syn::custom_keyword!(html);

    impl html {
        pub fn lit(self) -> LitStr {
            LitStr::new("html", self.span)
        }
    }
}

pub trait Syntax {}

pub struct Document<S: Syntax> {
    pub nodes: Nodes<S, ElementNode<S>>,
}

impl<S: Syntax> Generate for Document<S> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.nodes);
    }
}

pub trait Node<S: Syntax>: Generate + Sized {
    fn is_control(&self) -> bool;
}

pub enum ElementNode<S: Syntax> {
    Doctype(Doctype<S>),
    Element(Element<S>),
    Component(Component<S>),
    Literal(Literal),
    Control(Control<S, Self>),
    Expr(ParenExpr),
    Group(Group<S, Self>),
}

impl<S: Syntax> Node<S> for ElementNode<S> {
    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }
}

impl<S: Syntax> Generate for ElementNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Doctype(doctype) => g.push(doctype),
            Self::Element(element) => g.push(element),
            Self::Component(component) => g.push(component),
            Self::Literal(lit) => g.push_text_lit(&lit.lit_str()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_text(g),
            Self::Group(group) => g.push(group),
        }
    }
}

pub struct Doctype<S: Syntax> {
    pub lt_token: Token![<],
    pub bang_token: Token![!],
    pub doctype_token: kw::DOCTYPE,
    pub html_token: kw::html,
    pub gt_token: Token![>],
    pub phantom: PhantomData<S>,
}

impl<S: Syntax> Generate for Doctype<S> {
    fn generate(&self, g: &mut Generator) {
        g.push_lits(vec![
            LitStr::new("<", self.lt_token.span),
            LitStr::new("!", self.bang_token.span),
            self.doctype_token.lit(),
            LitStr::new(" ", Span::mixed_site()),
            self.html_token.lit(),
            LitStr::new(">", self.gt_token.span),
        ]);
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
            nodes: {
                let mut nodes = Vec::new();

                while !input.is_empty() {
                    nodes.push(input.parse()?);
                }

                nodes
            },
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
            if let Some(check) = attr.name.check() {
                el_checks.push_attribute(check);
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

impl<S: Syntax> Parse for Attribute<S>
where
    AttributeValueNode<S>: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            kind: if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;

                if let Some(toggle) = input.call(Toggle::parse_optional)? {
                    AttributeKind::Option(toggle)
                } else {
                    AttributeKind::Value {
                        value: input.parse()?,
                        toggle: input.call(Toggle::parse_optional)?,
                    }
                }
            } else {
                AttributeKind::Empty(input.call(Toggle::parse_optional)?)
            },
        })
    }
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
    pub fn check(&self) -> Option<AttributeCheck> {
        match self {
            Self::Data { .. } => None,
            Self::Namespace { namespace, .. } => Some(AttributeCheck::new(
                AttributeCheckKind::Namespace,
                namespace.ident_string(),
                namespace.spans(),
            )),
            Self::Symbol { symbol, .. } => Some(AttributeCheck::new(
                AttributeCheckKind::Symbol,
                symbol.ident_string(),
                vec![symbol.span()],
            )),
            Self::Normal(name) => Some(AttributeCheck::new(
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
                namespace, rest, ..
            } => {
                let mut lits = namespace.lits();
                lits.push(LitStr::new(":", Span::mixed_site()));
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
            Self::At(_) => "_at".to_string(),
            Self::Colon(_) => "_colon".to_string(),
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
        value: AttributeValueNode<S>,
        toggle: Option<Toggle>,
    },
    Empty(Option<Toggle>),
    Option(Toggle),
    ClassList(Vec<Class<S>>),
}

pub enum AttributeValueNode<S: Syntax> {
    Literal(Literal),
    Group(Group<S, Self>),
    Control(Control<S, Self>),
    Expr(ParenExpr),
    Ident(Ident),
}

impl<S: Syntax> AttributeValueNode<S> {
    pub fn parse_unquoted(input: ParseStream) -> syn::Result<Self>
    where
        Self: Parse,
    {
        if input.peek(Ident::peek_any) || input.peek(LitInt) {
            Ok(Self::Group(Group(Nodes {
                nodes: input
                    .parse::<UnquotedName>()?
                    .lits()
                    .into_iter()
                    .map(|lit| Self::Literal(Literal::Str(lit)))
                    .collect(),
                phantom: PhantomData,
            })))
        } else {
            input.parse()
        }
    }
}

impl<S: Syntax> Node<S> for AttributeValueNode<S> {
    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }
}

impl<S: Syntax> Generate for AttributeValueNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Literal(lit) => g.push_attribute_lit(&lit.lit_str()),
            Self::Group(block) => g.push(block),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_attribute(g),
            Self::Ident(ident) => g.push_attribute_expr(Paren::default(), ident),
        }
    }
}

pub struct Class<S: Syntax> {
    pub value: AttributeValueNode<S>,
    pub toggle: Option<Toggle>,
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
    pub dotdot: Option<Token![..]>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    fn generate(&self, g: &mut Generator) {
        let fields = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            let value = &attr.value_expr();

            quote!(#name: #value,)
        });

        let children = match &self.body {
            ElementBody::Normal { children, .. } => {
                let output_ident = Generator::output_ident();

                let block = g.block_with(Brace::default(), |g| {
                    g.push(children);
                });

                let lazy = quote! {
                    ::hypertext::Lazy({
                        extern crate alloc;

                        |#output_ident: &mut alloc::string::String|
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

        let default = self
            .dotdot
            .as_ref()
            .map(|dotdot| quote_spanned!(dotdot.span()=> ..::core::default::Default::default()))
            .unwrap_or_default();

        let init = quote! {
            #name {
                #(#fields)*
                #children
                #default
            }
        };

        g.push_text_expr(Paren::default(), &init);
    }
}

pub struct ComponentAttribute {
    pub name: Ident,
    pub value: ComponentAttributeValue,
}

impl ComponentAttribute {
    fn value_expr(&self) -> TokenStream {
        match &self.value {
            ComponentAttributeValue::Literal(lit) => match lit {
                Literal::Str(lit) => lit.to_token_stream(),
                Literal::Int(lit) => lit.to_token_stream(),
                Literal::Bool(lit) => lit.to_token_stream(),
                Literal::Float(lit) => lit.to_token_stream(),
            },
            ComponentAttributeValue::Ident(ident) => ident.to_token_stream(),
            ComponentAttributeValue::Expr(expr) => {
                let mut tokens = TokenStream::new();

                expr.paren_token.surround(&mut tokens, |tokens| {
                    expr.expr.to_tokens(tokens);
                });

                tokens
            }
        }
    }
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
    Literal(Literal),
    Ident(Ident),
    Expr(ParenExpr),
}

impl Parse for ComponentAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Ident) {
            input.parse().map(Self::Ident)
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
                NameFragment::Colon(_) | NameFragment::Dot(_) => {
                    unreachable!(
                        "unquoted name idents should only contain identifiers, int literals, and hyphens"
                    );
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
            || input.peek(Token![.])
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
    Dot(Token![.]),
}

impl NameFragment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Number(num) => num.span(),
            Self::Hyphen(hyphen) => hyphen.span(),
            Self::Colon(colon) => colon.span(),
            Self::Dot(dot) => dot.span(),
        }
    }

    const fn is_punct(&self) -> bool {
        matches!(self, Self::Hyphen(_) | Self::Colon(_) | Self::Dot(_))
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
        } else if lookahead.peek(Token![.]) {
            input.parse().map(Self::Dot)
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
            Self::Dot(_) => f.write_str("."),
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
