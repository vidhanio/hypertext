#![expect(clippy::struct_field_names, clippy::large_enum_variant)]

mod basics;
mod component;
mod control;
pub mod generate;
mod syntaxes;

use std::{borrow::Cow, convert::Infallible, marker::PhantomData};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Error, Ident, LitBool, LitChar, LitFloat, LitInt, LitStr, Token, braced, bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote_spanned,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
};

pub use self::syntaxes::{Maud, Rsx};
use self::{
    basics::{Literal, UnquotedName},
    component::Component,
    control::Control,
    generate::{
        AnyBlock, AttributeCheck, AttributeCheckKind, ElementCheck, ElementKind, Generate,
        Generator,
    },
};

mod kw {
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

pub type Document<S> = Many<Node<S>>;

pub trait Context: Generate<Context = Self> {
    fn is_control(&self) -> bool;

    fn marker_type() -> TokenStream;

    fn escape(s: &str) -> Cow<'_, str>;
}

impl Context for Infallible {
    fn is_control(&self) -> bool {
        #[expect(clippy::uninhabited_references)]
        match *self {}
    }

    fn marker_type() -> TokenStream {
        TokenStream::new()
    }

    fn escape(s: &str) -> Cow<'_, str> {
        Cow::Borrowed(s)
    }
}

pub enum Node<S: Syntax> {
    Doctype(Doctype<S>),
    Element(Element<S>),
    Component(Component<S>),
    Literal(Literal),
    Control(Control<Self>),
    Expr(ParenExpr<Self>),
    DisplayExpr(DisplayExpr<Self>),
    DebugExpr(DebugExpr<Self>),
    Group(Group<Self>),
}

impl<S: Syntax> Context for Node<S> {
    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn marker_type() -> TokenStream {
        quote!(::hypertext::context::Node)
    }

    fn escape(s: &str) -> Cow<'_, str> {
        html_escape::encode_text(s)
    }
}

impl<S: Syntax> Generate for Node<S> {
    type Context = Self;

    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Doctype(doctype) => g.push(doctype),
            Self::Element(element) => g.push(element),
            Self::Component(component) => g.push(component),
            Self::Literal(lit) => g.push_escaped_lit::<Self::Context>(&lit.lit_str()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => g.push(expr),
            Self::DisplayExpr(display_expr) => g.push(display_expr),
            Self::DebugExpr(debug_expr) => g.push(debug_expr),
            Self::Group(group) => g.push(group),
        }
    }
}

pub struct Doctype<S: Syntax> {
    lt_token: Token![<],
    bang_token: Token![!],
    doctype_token: kw::DOCTYPE,
    html_token: kw::html,
    gt_token: Token![>],
    phantom: PhantomData<S>,
}

impl<S: Syntax> Generate for Doctype<S> {
    type Context = Node<S>;

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

pub struct ParenExpr<C: Context> {
    paren_token: Paren,
    expr: TokenStream,
    phantom: PhantomData<C>,
}

impl<C: Context> Parse for ParenExpr<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            paren_token: parenthesized!(content in input),
            expr: content.parse()?,
            phantom: PhantomData,
        })
    }
}

impl<C: Context> Generate for ParenExpr<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        g.push_expr::<Self::Context>(self.paren_token, &self.expr);
    }
}

impl<C: Context> ToTokens for ParenExpr<C> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}

pub struct DisplayExpr<C: Context> {
    percent_token: Token![%],
    paren_expr: ParenExpr<C>,
}

impl<C: Context> DisplayExpr<C> {
    fn wrapped_expr(&self) -> TokenStream {
        let wrapper = quote_spanned!(self.percent_token.span=> Displayed);
        let mut new_paren_expr = TokenStream::new();
        self.paren_expr
            .paren_token
            .surround(&mut new_paren_expr, |tokens| {
                quote_spanned!(self.paren_expr.paren_token.span=> &).to_tokens(tokens);
                self.paren_expr.to_tokens(tokens);
            });

        quote!(::hypertext::#wrapper #new_paren_expr)
    }
}

impl<C: Context> Parse for DisplayExpr<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            percent_token: input.parse()?,
            paren_expr: input.parse()?,
        })
    }
}

impl<C: Context> Generate for DisplayExpr<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        g.push_expr::<Self::Context>(self.paren_expr.paren_token, self.wrapped_expr());
    }
}

pub struct DebugExpr<C: Context> {
    question_token: Token![?],
    expr: ParenExpr<C>,
}

impl<C: Context> DebugExpr<C> {
    fn wrapped_expr(&self) -> TokenStream {
        let wrapper = quote_spanned!(self.question_token.span=> Debugged);
        let mut new_paren_expr = TokenStream::new();
        self.expr
            .paren_token
            .surround(&mut new_paren_expr, |tokens| {
                quote_spanned!(self.expr.paren_token.span=> &).to_tokens(tokens);
                self.expr.to_tokens(tokens);
            });

        quote!(::hypertext::#wrapper #new_paren_expr)
    }
}

impl<C: Context> Parse for DebugExpr<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            question_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl<C: Context> Generate for DebugExpr<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        g.push_expr::<Self::Context>(self.expr.paren_token, self.wrapped_expr());
    }
}

pub struct Group<C: Context>(Many<C>);

impl Parse for Group<AttributeValue> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        Ok(Self(content.parse()?))
    }
}

impl<C: Context> Generate for Group<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        g.push(&self.0);
    }
}

pub struct Many<C: Context>(Vec<C>);

impl<C: Context> Many<C> {
    fn block(&self, g: &mut Generator, brace_token: Brace) -> AnyBlock {
        g.block_with(brace_token, |g| {
            g.push_all(&self.0);
        })
    }
}

impl<C: Context + Parse> Parse for Many<C> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self({
            let mut children = Vec::new();

            while !input.is_empty() {
                children.push(input.parse()?);
            }

            children
        }))
    }
}

impl<C: Context> Generate for Many<C> {
    type Context = C;

    fn generate(&self, g: &mut Generator) {
        if self.0.iter().any(Context::is_control) {
            g.push_in_block(Brace::default(), |g| g.push_all(&self.0));
        } else {
            g.push_all(&self.0);
        }
    }
}

pub struct Element<S: Syntax> {
    name: UnquotedName,
    attrs: Vec<Attribute>,
    body: ElementBody<S>,
}

impl<S: Syntax> Generate for Element<S> {
    type Context = Node<S>;

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
                    el_checks.set_closing_spans(closing_name.spans());
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
        children: Many<Node<S>>,
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

pub struct Attribute {
    name: AttributeName,
    kind: AttributeKind,
}

impl Attribute {
    fn parse_id(input: ParseStream) -> syn::Result<Self> {
        let pound_token = input.parse::<Token![#]>()?;
        Ok(Self {
            name: parse_quote_spanned!(pound_token.span()=> id),
            kind: AttributeKind::Value {
                value: input.call(AttributeValue::parse_unquoted)?,
                toggle: None,
            },
        })
    }

    fn parse_class_list(input: ParseStream) -> syn::Result<Self> {
        let dot_token = input.fork().parse::<Token![.]>()?;
        let mut classes = Vec::new();

        while input.peek(Token![.]) {
            classes.push(input.parse()?);
        }

        Ok(Self {
            name: parse_quote_spanned!(dot_token.span()=> class),
            kind: AttributeKind::ClassList(classes),
        })
    }
}

impl Parse for Attribute {
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

impl Generate for Attribute {
    type Context = AttributeValue;

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
                        g.push_expr::<Self::Context>(Paren::default(), &value);
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
                    class.generate(g, i);
                }

                g.push_str("\"");
            }
        }
    }
}

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
    Unchecked(LitStr),
}

impl AttributeName {
    fn check(&self) -> Option<AttributeCheck> {
        match self {
            Self::Data { .. } | Self::Unchecked(_) => None,
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
            Self::Unchecked(lit) => vec![lit.clone()],
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
        } else if lookahead.peek(LitStr) {
            let s = input.parse::<LitStr>()?;
            let value = s.value();

            for c in value.chars() {
                if c.is_whitespace() {
                    return Err(Error::new_spanned(
                        &s,
                        "Attribute names cannot contain whitespace",
                    ));
                } else if c.is_control() {
                    return Err(Error::new_spanned(
                        &s,
                        "Attribute names cannot contain control characters",
                    ));
                } else if c == '>' || c == '/' || c == '=' {
                    return Err(Error::new_spanned(
                        &s,
                        format!("Attribute names cannot contain '{c}' characters"),
                    ));
                } else if c == '"' || c == '\'' {
                    return Err(Error::new_spanned(
                        &s,
                        "Attribute names cannot contain quotes",
                    ));
                }
            }

            Ok(Self::Unchecked(s))
        } else {
            Err(lookahead.error())
        }
    }
}

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

pub enum AttributeKind {
    Value {
        value: AttributeValue,
        toggle: Option<Toggle>,
    },
    Empty(Option<Toggle>),
    Option(Toggle),
    ClassList(Vec<Class>),
}

pub enum AttributeValue {
    Literal(Literal),
    Group(Group<Self>),
    Control(Control<Self>),
    Expr(ParenExpr<Self>),
    DisplayExpr(DisplayExpr<Self>),
    DebugExpr(DebugExpr<Self>),
    Ident(Ident),
}

impl AttributeValue {
    fn parse_unquoted(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident::peek_any) || input.peek(LitInt) {
            Ok(Self::Group(Group(Many(
                input
                    .call(UnquotedName::parse_attr_value)?
                    .lits()
                    .into_iter()
                    .map(|lit| Self::Literal(Literal::Str(lit)))
                    .collect(),
            ))))
        } else {
            input.parse()
        }
    }
}

impl Context for AttributeValue {
    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn marker_type() -> TokenStream {
        quote!(::hypertext::context::AttributeValue)
    }

    fn escape(s: &str) -> Cow<'_, str> {
        html_escape::encode_double_quoted_attribute(s)
    }
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr)
            || lookahead.peek(LitInt)
            || lookahead.peek(LitBool)
            || lookahead.peek(LitFloat)
            || lookahead.peek(LitChar)
        {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Group)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else if lookahead.peek(Token![%]) {
            input.parse().map(Self::DisplayExpr)
        } else if lookahead.peek(Token![?]) {
            input.parse().map(Self::DebugExpr)
        } else if lookahead.peek(Ident) {
            input.parse().map(Self::Ident)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Generate for AttributeValue {
    type Context = Self;

    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Literal(lit) => g.push_escaped_lit::<Self::Context>(&lit.lit_str()),
            Self::Group(block) => g.push(block),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => g.push(expr),
            Self::DisplayExpr(display_expr) => g.push(display_expr),
            Self::DebugExpr(debug_expr) => g.push(debug_expr),
            Self::Ident(ident) => g.push_expr::<Self::Context>(Paren::default(), ident),
        }
    }
}

pub enum Class {
    Value {
        value: AttributeValue,
        toggle: Option<Toggle>,
    },
    Option(Toggle),
}

impl Class {
    fn generate(&self, g: &mut Generator, index: usize) {
        match self {
            Self::Value { value, toggle } => {
                if let Some(toggle) = toggle {
                    g.push_conditional(toggle.parenthesized(), |g| {
                        if index > 0 {
                            g.push_str(" ");
                        }
                        g.push(value);
                    });
                } else {
                    if index > 0 {
                        g.push_str(" ");
                    }
                    g.push(value);
                }
            }
            Self::Option(option) => {
                let option_expr = &option.expr;
                let value = Ident::new("value", Span::mixed_site());

                g.push_conditional(
                    quote!(let ::core::option::Option::Some(#value) = (#option_expr)),
                    |g| {
                        if index > 0 {
                            g.push_str(" ");
                        }
                        g.push_expr::<AttributeValue>(Paren::default(), &value);
                    },
                );
            }
        }
    }
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![.]>()?;

        if input.peek(Bracket) {
            Ok(Self::Option(input.parse()?))
        } else {
            Ok(Self::Value {
                value: input.call(AttributeValue::parse_unquoted)?,
                toggle: input.call(Toggle::parse_optional)?,
            })
        }
    }
}

pub struct Toggle {
    bracket_token: Bracket,
    expr: TokenStream,
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

        quote! {
            {
                #[allow(unused_parens)]
                #tokens
            }
        }
    }

    fn parse_optional(input: ParseStream) -> syn::Result<Option<Self>> {
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
