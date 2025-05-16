use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Nothing, Parse, ParseStream},
    parse_quote_spanned,
    spanned::Spanned,
    token::{Brace, Paren},
};

use crate::node::{
    Attribute, AttributeKind, Class, Component, ControlSyntax, Element, ElementBody, ElementNode,
    Group, Markup, Node, QuotedValueNode, Syntax, Toggle, UnquotedName, UnquotedValueNode,
};

pub struct Maud;

impl Syntax for Maud {
    type NodeSeparator = Nothing;
}

impl ControlSyntax for Maud {
    type ControlToken = Token![@];
}

impl Parse for Markup<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            doctype: {
                syn::custom_keyword!(DOCTYPE);

                if input.peek(Token![!]) && input.peek2(DOCTYPE) {
                    input.parse::<Token![!]>()?;

                    Some(input.parse::<DOCTYPE>()?.span())
                } else {
                    None
                }
            },
            nodes: input.parse()?,
        })
    }
}

impl Parse for ElementNode<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            if input.fork().parse::<UnquotedName>()?.is_component() {
                input.parse().map(Self::Component)
            } else {
                input.parse().map(Self::Element)
            }
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

impl<N: Node<Maud> + Parse> Parse for Group<Maud, N> {
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
                    attrs.push(input.call(Attribute::parse_normal)?);
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

impl Attribute<Maud> {
    fn parse_id(input: ParseStream) -> syn::Result<Self> {
        let pound_token = input.parse::<Token![#]>()?;
        Ok(Self {
            name: parse_quote_spanned!(pound_token.span()=> id),
            kind: AttributeKind::Value {
                value: input.parse()?,
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

    fn parse_normal(input: ParseStream) -> syn::Result<Self> {
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

impl Parse for Class<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![.]>()?;
        Ok(Self {
            value: input.parse()?,
            toggle: input.call(Toggle::parse_optional)?,
        })
    }
}

impl Parse for UnquotedValueNode<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            input.parse().map(Self::UnquotedName)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Str)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Group)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for QuotedValueNode<Maud> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Group)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Control)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
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

                while !(input.peek(Token![;]) || input.peek(Brace)) {
                    attrs.push(input.parse()?);
                }

                attrs
            },
            body: input.parse()?,
        })
    }
}
