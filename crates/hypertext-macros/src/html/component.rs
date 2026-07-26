use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Ident, Lit, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

use super::{AttributeValue, ElementBody, Generate, Generator, ParenExpr, Syntax};
use crate::html::Node;

/// How the children block is handed to a component's `children` setter.
pub enum ChildrenMode {
    /// `ref`: pass `&Lazy<_>`, so the component borrows the children.
    ///
    /// Required by components storing children behind a reference, such as
    /// `children: &dyn Renderable`.
    Ref(Token![ref]),

    /// `move`: pass `Lazy<_>` by value, so the component owns it.
    ///
    /// Required by components storing children by value, such as
    /// `children: Lazy<fn(&mut Buffer)>`.
    Move(Token![move]),
}

impl ChildrenMode {
    /// Whether children are passed by value, falling back to the
    /// `children-move` feature when the call site does not say.
    const fn is_move(this: Option<&Self>) -> bool {
        match this {
            Some(Self::Move(_)) => true,
            Some(Self::Ref(_)) => false,
            None => cfg!(feature = "children-move"),
        }
    }

    /// Parses a trailing `ref`/`move` marker from a component's attribute
    /// list.
    ///
    /// Both are Rust keywords, so they can never collide with an attribute
    /// name: a struct field cannot be called `ref` or `move` either.
    pub fn parse_opt(input: ParseStream) -> syn::Result<Option<Self>> {
        let mode = if input.peek(Token![ref]) {
            Self::Ref(input.parse()?)
        } else if input.peek(Token![move]) {
            Self::Move(input.parse()?)
        } else {
            return Ok(None);
        };

        if input.peek(Ident::peek_any) || input.peek(Token![ref]) || input.peek(Token![move]) {
            return Err(input.error(format!(
                "`{}` must be the last attribute of the component",
                mode.as_str()
            )));
        }

        Ok(Some(mode))
    }

    const fn as_str(&self) -> &'static str {
        match self {
            Self::Ref(_) => "ref",
            Self::Move(_) => "move",
        }
    }
}

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute>,
    pub children_mode: Option<ChildrenMode>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Component<S> {
    /// Creates a component, rejecting a `ref`/`move` marker on a component
    /// that has no children block for it to apply to.
    pub fn new(
        name: Ident,
        attrs: Vec<ComponentAttribute>,
        children_mode: Option<ChildrenMode>,
        body: ElementBody<S>,
    ) -> syn::Result<Self> {
        if let (Some(mode), ElementBody::Void { .. }) = (&children_mode, &body) {
            let span = match mode {
                ChildrenMode::Ref(token) => token.span,
                ChildrenMode::Move(token) => token.span,
            };

            return Err(syn::Error::new(
                span,
                format!("`{}` requires a children block to apply to", mode.as_str()),
            ));
        }

        Ok(Self {
            name,
            attrs,
            children_mode,
            body,
        })
    }
}

impl<S: Syntax> Generate for Component<S> {
    type Context = Node<S>;

    fn generate(&self, g: &mut Generator) {
        let props = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            attr.value_expr()
                .map_or_else(|| quote!(.#name(#name)), |value| quote!(.#name(#value)))
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
                let ampersand = if ChildrenMode::is_move(self.children_mode.as_ref()) {
                    None
                } else {
                    Some(<Token![&]>::default())
                };

                quote!(
                    .#children_ident(#ampersand #lazy)
                )
            }
            ElementBody::Void { .. } => quote!(),
        };

        let name = &self.name;

        let init = quote! {
            #name::builder()
                #(#props)*
                #children
                .build()
        };

        g.push_expr::<Self::Context>(Paren::default(), &init);
    }
}

pub struct ComponentAttribute {
    name: Ident,
    value: Option<ComponentAttributeValue>,
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

                quote! {
                    {
                        #[allow(unused_parens)]
                        #tokens
                    }
                }
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
