use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Ident, Lit, Token,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Paren},
};

use super::{AttributeValue, ElementBody, Generate, Generator, Node, ParenExpr, Syntax};

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute>,
    pub dotdot: Option<Token![..]>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    type Context = Node<S>;

    fn generate(&self, g: &mut Generator) {
        let fields = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            attr.value_expr()
                .map_or_else(|| quote!(#name,), |value| quote!(#name: #value,))
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

        g.push_expr::<Self::Context>(Paren::default(), &init);
    }
}

pub struct ComponentAttribute {
    pub name: Ident,
    pub value: Option<ComponentAttributeValue>,
}

impl ComponentAttribute {
    fn value_expr(&self) -> Option<TokenStream> {
        self.value.as_ref().map(|value| match value {
            ComponentAttributeValue::Literal(lit) => lit.to_token_stream(),
            ComponentAttributeValue::Ident(ident) => ident.to_token_stream(),
            ComponentAttributeValue::Expr(expr) => {
                let mut tokens = TokenStream::new();

                expr.paren_token.surround(&mut tokens, |tokens| {
                    expr.expr.to_tokens(tokens);
                });

                tokens
            }
        })
    }
}

impl Parse for ComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            value: {
                if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;

                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

pub enum ComponentAttributeValue {
    Literal(Lit),
    Ident(Ident),
    Expr(ParenExpr<AttributeValue>),
}

impl Parse for ComponentAttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Lit) {
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
