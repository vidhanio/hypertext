use std::borrow::Cow;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Error, Ident, LitBool, LitChar, LitFloat, LitInt, LitStr, Token, bracketed,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_quote_spanned,
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
};

use super::{Control, DebugExpr, DisplayExpr, Group, Literal, Many, ParenExpr, UnquotedName, kw};
use crate::{AttributeCheck, AttributeCheckKind, Context, Generate, Generator};

pub struct Attribute {
    pub name: AttributeName,
    pub kind: AttributeKind,
}

impl Attribute {
    pub fn parse_id(input: ParseStream) -> syn::Result<Self> {
        let pound_token = input.parse::<Token![#]>()?;
        Ok(Self {
            name: parse_quote_spanned!(pound_token.span()=> id),
            kind: AttributeKind::Value {
                value: input.call(AttributeValue::parse_unquoted)?,
                toggle: None,
            },
        })
    }

    pub fn parse_class_list(input: ParseStream) -> syn::Result<Self> {
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
    pub fn check(&self) -> Option<AttributeCheck> {
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

    pub fn lits(&self) -> Vec<LitStr> {
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
    pub fn lit(&self) -> LitStr {
        match self {
            Self::At(token) => LitStr::new("@", token.span()),
            Self::Colon(token) => LitStr::new(":", token.span()),
        }
    }

    pub fn ident_string(&self) -> String {
        match self {
            Self::At(_) => "_at".to_string(),
            Self::Colon(_) => "_colon".to_string(),
        }
    }

    pub fn span(&self) -> Span {
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
    pub fn parse_unquoted(input: ParseStream) -> syn::Result<Self> {
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
    pub bracket_token: Bracket,
    pub expr: TokenStream,
}

impl Toggle {
    pub fn parenthesized(&self) -> TokenStream {
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
