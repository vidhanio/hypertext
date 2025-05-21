use std::marker::PhantomData;

use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token, braced, custom_punctuation,
    ext::IdentExt,
    parse::{Parse, ParseStream, discouraged::Speculative},
    token::{Brace, Paren},
};

use crate::node::{
    AttributeValueNode, Component, Doctype, Document, Element, ElementBody, ElementNode, Group,
    Literal, NameFragment, Nodes, Syntax, UnquotedName,
};

pub struct Rsx;

impl Syntax for Rsx {}

custom_punctuation!(FragmentOpen, <>);
custom_punctuation!(FragmentClose, </>);
custom_punctuation!(OpenTagSolidusEnd, />);
custom_punctuation!(CloseTagStart, </);

impl Parse for Document<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: input.parse()?,
        })
    }
}

impl ElementNode<Rsx> {
    fn parse_component(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;

        let name = input.parse::<Ident>()?;

        let mut attrs = Vec::new();

        while !(input.peek(Token![..]) || input.peek(Token![>]) || input.peek(OpenTagSolidusEnd)) {
            attrs.push(input.parse()?);
        }

        let dotdot = input.parse()?;

        let solidus = input.parse::<Option<Token![/]>>()?;
        input.parse::<Token![>]>()?;

        if solidus.is_some() {
            Ok(Self::Component(Component {
                name,
                attrs,
                dotdot,
                body: ElementBody::Void,
            }))
        } else {
            let mut children = Vec::new();

            while !input.peek(CloseTagStart) {
                if input.is_empty() {
                    children.insert(
                        0,
                        Self::Component(Component {
                            name,
                            attrs,
                            dotdot,
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
            fork.parse::<CloseTagStart>()?;
            let closing_name = fork.parse::<Ident>()?;
            if closing_name == name {
                input.advance_to(&fork);
            } else {
                children.insert(
                    0,
                    Self::Component(Component {
                        name,
                        attrs,
                        dotdot,
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
                dotdot,
                body: ElementBody::Normal {
                    children: Nodes {
                        nodes: children,
                        phantom: PhantomData,
                    },
                    closing_name: Some(UnquotedName(vec![NameFragment::Ident(closing_name)])),
                },
            }))
        }
    }

    fn parse_element(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;

        let name = input.parse()?;

        let mut attrs = Vec::new();

        while !(input.peek(Token![>]) || (input.peek(OpenTagSolidusEnd))) {
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

            while !(input.peek(CloseTagStart)) {
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
            fork.parse::<CloseTagStart>()?;
            let closing_name = fork.parse()?;
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
                input.parse().map(Self::Group)
            } else if lookahead.peek(Ident::peek_any) {
                if fork.parse::<UnquotedName>()?.is_component() {
                    input.call(Self::parse_component)
                } else {
                    input.call(Self::parse_element)
                }
            } else if lookahead.peek(Token![!]) {
                input.parse().map(Self::Doctype)
            } else {
                Err(lookahead.error())
            }
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else if lookahead.peek(LitStr)
            || lookahead.peek(LitInt)
            || lookahead.peek(LitBool)
            || lookahead.peek(LitFloat)
        {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;

            let ident_string = if input.peek(Ident::peek_any)
                || input.peek(LitInt)
                || input.peek(LitBool)
                || input.peek(LitFloat)
            {
                format!("{ident} ")
            } else {
                ident.to_string()
            };

            Ok(Self::Literal(Literal::Str(LitStr::new(
                &ident_string,
                ident.span(),
            ))))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Doctype<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            lt_token: input.parse()?,
            bang_token: input.parse()?,
            doctype_token: input.parse()?,
            html_token: input.parse()?,
            gt_token: input.parse()?,
            phantom: PhantomData,
        })
    }
}

impl Parse for Group<Rsx, ElementNode<Rsx>> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<FragmentOpen>()?;

        let mut nodes = Vec::new();

        while !input.peek(FragmentClose) {
            nodes.push(input.parse()?);
        }

        input.parse::<FragmentClose>()?;

        Ok(Self(Nodes {
            nodes,
            phantom: PhantomData,
        }))
    }
}

impl Parse for Group<Rsx, AttributeValueNode<Rsx>> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);

        Ok(Self(content.parse()?))
    }
}

impl Parse for AttributeValueNode<Rsx> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr)
            || lookahead.peek(LitInt)
            || lookahead.peek(LitBool)
            || lookahead.peek(LitFloat)
        {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Group)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else if lookahead.peek(Ident) {
            input.parse().map(Self::Ident)
        } else {
            Err(lookahead.error())
        }
    }
}
