use std::{iter, marker::PhantomData};

use quote::ToTokens;
use syn::{
    Ident, LitBool, LitInt, LitStr, Token,
    ext::IdentExt,
    parse::{Nothing, Parse, ParseStream, discouraged::Speculative},
    punctuated::Pair,
    spanned::Spanned,
    token::Brace,
};

use crate::{
    generate::AnyBlock,
    node::{
        AnyExpr, Attribute, AttributeKind, Component, ComponentAttribute, ComponentAttributeValue,
        ControlSyntax, Element, ElementBody, ElementNode, Group, Literal, Markup, NameFragment,
        Nodes, QuotedValueNode, Syntax, UnquotedName, UnquotedValueNode,
    },
};

pub struct Rsx;

impl Syntax for Rsx {
    type NodeSeparator = Nothing;
}

impl ControlSyntax for Rsx {
    type ControlToken = Token![@];
}

impl Parse for Markup<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            doctype: {
                syn::custom_keyword!(DOCTYPE);
                syn::custom_keyword!(html);

                if input.peek(Token![<]) && input.peek2(Token![!]) {
                    input.parse::<Token![<]>()?;
                    input.parse::<Token![!]>()?;
                    let doctype = input.parse::<DOCTYPE>()?;
                    input.parse::<html>()?;
                    input.parse::<Token![>]>()?;

                    Some(doctype.span())
                } else {
                    None
                }
            },
            nodes: input.parse()?,
        })
    }
}

impl ElementNode<Rsx> {
    fn parse_fragment(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![>]>()?;

        let mut nodes = Vec::new();

        while !(input.peek(Token![<]) && input.peek2(Token![/]) && input.peek3(Token![>])) {
            nodes.push(input.parse()?);
        }

        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        input.parse::<Token![>]>()?;

        Ok(Self::Group(Group(Nodes {
            nodes,
            phantom: PhantomData,
        })))
    }

    fn parse_component(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;

        let name = input.parse::<Ident>()?;

        let mut attrs = Vec::new();

        while !(input.peek(Token![>]) || (input.peek(Token![/]) && input.peek2(Token![>]))) {
            attrs.push(input.parse()?);
        }

        let solidus = input.parse::<Option<Token![/]>>()?;
        input.parse::<Token![>]>()?;

        if solidus.is_some() {
            Ok(Self::Component(Component {
                name,
                attrs,
                body: ElementBody::Void,
            }))
        } else {
            let mut children = Vec::new();

            while !(input.peek(Token![<]) && input.peek2(Token![/])) {
                if input.is_empty() {
                    children.insert(
                        0,
                        Self::Component(Component {
                            name,
                            attrs,
                            body: ElementBody::Void,
                        }),
                    );

                    return Ok(Self::Group(Group(Nodes {
                        nodes: children,
                        phantom: PhantomData,
                    })));
                }

                children.push(input.parse()?);
            }

            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            fork.parse::<Token![/]>()?;
            let closing_name = fork.parse::<Ident>()?;
            if closing_name == name {
                input.advance_to(&fork);
            } else {
                children.insert(
                    0,
                    Self::Component(Component {
                        name,
                        attrs,
                        body: ElementBody::Void,
                    }),
                );

                return Ok(Self::Group(Group(Nodes {
                    nodes: children,
                    phantom: PhantomData,
                })));
            }
            input.parse::<Token![>]>()?;

            Ok(Self::Component(Component {
                name,
                attrs,
                body: ElementBody::Normal {
                    children: Nodes {
                        nodes: children,
                        phantom: PhantomData,
                    },
                    closing_name: Some(UnquotedName(
                        iter::once(Pair::End(NameFragment::Ident(closing_name))).collect(),
                    )),
                },
            }))
        }
    }

    fn parse_element(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        
        let name = input.parse::<UnquotedName>()?;

        let mut attrs = Vec::new();

        while !(input.peek(Token![>]) || (input.peek(Token![/]) && input.peek2(Token![>]))) {
            attrs.push(input.parse()?);
        }

        let solidus = input.parse::<Option<Token![/]>>()?;
        input.parse::<Token![>]>()?;

        if solidus.is_some() {
            Ok(Self::Element(Element {
                name,
                attrs,
                body: ElementBody::Void,
            }))
        } else {
            let mut children = Vec::new();

            while !(input.peek(Token![<]) && input.peek2(Token![/])) {
                if input.is_empty() {
                    children.insert(
                        0,
                        Self::Element(Element {
                            name,
                            attrs,
                            body: ElementBody::Void,
                        }),
                    );

                    return Ok(Self::Group(Group(Nodes {
                        nodes: children,
                        phantom: PhantomData,
                    })));
                }
                children.push(input.parse()?);
            }

            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            fork.parse::<Token![/]>()?;
            let closing_name = fork.parse::<UnquotedName>()?;
            if closing_name == name {
                input.advance_to(&fork);
            } else {
                children.insert(
                    0,
                    Self::Element(Element {
                        name,
                        attrs,
                        body: ElementBody::Void,
                    }),
                );

                return Ok(Self::Group(Group(Nodes {
                    nodes: children,
                    phantom: PhantomData,
                })));
            }
            input.parse::<Token![>]>()?;

            Ok(Self::Element(Element {
                name,
                attrs,
                body: ElementBody::Normal {
                    children: Nodes {
                        nodes: children,
                        phantom: PhantomData,
                    },
                    closing_name: Some(closing_name),
                },
            }))
        }
    }
}

impl Parse for ElementNode<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![<]) {
            let fork = input.fork();
            fork.parse::<Token![<]>()?;
            let lookahead = fork.lookahead1();
            if lookahead.peek(Token![>]) {
                input.call(Self::parse_fragment)
            } else if lookahead.peek(Ident::peek_any) {
                if fork.parse::<UnquotedName>()?.is_component() {
                    input.call(Self::parse_component)
                } else {
                    input.call(Self::parse_element)
                }
            } else {
                Err(lookahead.error())
            }
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Expr)
        } else if lookahead.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;

            let ident_string = if input.peek(Token![<]) || input.is_empty() {
                ident.to_string()
            } else {
                format!("{ident} ")
            };

            Ok(Self::Literal(Literal::Str(LitStr::new(
                &ident_string,
                ident.span(),
            ))))
        } else if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for AnyExpr<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse::<AnyBlock>()?.to_token_stream(),
            phantom: PhantomData,
        })
    }
}

impl Parse for Attribute<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<UnquotedName>()?;

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            Ok(Self {
                name,
                kind: AttributeKind::Value {
                    value: input.parse()?,
                    toggle: None,
                },
            })
        } else {
            Ok(Self {
                name,
                kind: AttributeKind::Empty(None),
            })
        }
    }
}

impl Parse for UnquotedValueNode<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            input.parse().map(Self::UnquotedName)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for QuotedValueNode<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ComponentAttribute<Rsx> {
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

impl Parse for ComponentAttributeValue<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::UnquotedName)
        } else if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}
