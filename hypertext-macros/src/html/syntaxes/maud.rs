use std::marker::PhantomData;

use proc_macro2::Span;
use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

use crate::html::{
    Attribute, Component, Doctype, Element, ElementBody, ElementNode, Group, Syntax, UnquotedName,
    kw,
};

pub struct Maud;

impl Syntax for Maud {}

impl Parse for ElementNode<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            if input.fork().parse::<UnquotedName>()?.is_component() {
                input.parse().map(Self::Component)
            } else {
                input.parse().map(Self::Element)
            }
        } else if lookahead.peek(Token![!]) {
            input.parse().map(Self::Doctype)
        } else if lookahead.peek(LitStr)
            || lookahead.peek(LitInt)
            || lookahead.peek(LitBool)
            || lookahead.peek(LitFloat)
        {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Group)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Doctype<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lt_token: Token![<](Span::mixed_site()),
            bang_token: input.parse()?,
            doctype_token: input.parse()?,
            html_token: kw::html(Span::mixed_site()),
            gt_token: Token![>](Span::mixed_site()),
            phantom: PhantomData,
        })
    }
}

impl Parse for Group<ElementNode<Maud>> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        Ok(Self(content.parse()?))
    }
}

impl Parse for Element<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attrs: {
                let mut attrs = Vec::new();

                if input.peek(Token![#]) {
                    attrs.push(input.call(Attribute::parse_id)?);
                }

                if input.peek(Token![.]) {
                    attrs.push(input.call(Attribute::parse_class_list)?);
                }

                while !(input.peek(Token![;]) || input.peek(Brace)) {
                    attrs.push(input.parse()?);
                }

                attrs
            },
            body: input.parse()?,
        })
    }
}

impl Parse for ElementBody<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            let content;
            braced!(content in input);
            content.parse().map(|children| Self::Normal {
                children,
                closing_name: None,
            })
        } else if lookahead.peek(Token![;]) {
            input.parse::<Token![;]>().map(|_| Self::Void)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Component<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attrs: {
                let mut attrs = Vec::new();

                while !(input.peek(Token![..]) || input.peek(Token![;]) || input.peek(Brace)) {
                    attrs.push(input.parse()?);
                }

                attrs
            },
            dotdot: input.parse()?,
            body: input.parse()?,
        })
    }
}
