use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Ident, LitBool, LitChar, LitFloat, LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Paren},
};

use super::{ElementBody, Generate, Generator, Literal, ParenExpr, Syntax};
use crate::{AttributeValueNode, NodeType};

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute>,
    pub dotdot: Option<Token![..]>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    const NODE_TYPE: NodeType = NodeType::Element;

    fn generate(&self, g: &mut Generator) {
        let fields = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            let value = &attr.value_expr();

            quote!(#name: #value,)
        });

        let children = match &self.body {
            ElementBody::Normal { children, .. } => {
                let buffer_ident = Generator::buffer_ident();

                let block = g.block_with(Brace::default(), |g| {
                    g.push(children);
                });

                let lazy = quote! {
                    ::hypertext::Lazy::dangerously_create(
                        |#buffer_ident: &mut ::hypertext::Buffer|
                            #block
                    )
                };

                let children_ident = Ident::new("children", self.name.span());

                quote!(
                    #children_ident: #lazy,
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

        g.push_expr(Paren::default(), Self::NODE_TYPE, &init);
    }
}

pub struct ComponentAttribute {
    name: Ident,
    value: ComponentAttributeValue,
}

impl ComponentAttribute {
    fn value_expr(&self) -> TokenStream {
        match &self.value {
            ComponentAttributeValue::Literal(lit) => lit.to_token_stream(),
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
    Expr(ParenExpr<AttributeValueNode>),
}

impl Parse for ComponentAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr)
            || lookahead.peek(LitInt)
            || lookahead.peek(LitBool)
            || lookahead.peek(LitFloat)
            || lookahead.peek(LitChar)
        {
            input.call(Literal::parse_any).map(Self::Literal)
        } else if lookahead.peek(Ident) {
            input.parse().map(Self::Ident)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Expr)
        } else {
            Err(lookahead.error())
        }
    }
}
