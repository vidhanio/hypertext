use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::{
    Ident, LitStr,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::{Brace, Paren},
};

use super::{
    Attribute, AttributeKind, AttributeValue, Class, ElementBody, Generate, Generator, Syntax,
    generate::AnyBlock,
};
use crate::html::{Node, basics::Literal};

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    type Context = Node<S>;

    fn generate(&self, g: &mut Generator) {
        let mut owned: Vec<TokenStream> = vec![];
        let props: Vec<TokenStream> = self
            .attrs
            .iter()
            .map(|attr| attr.kind_expr(g, &mut owned))
            .collect();

        let children = match &self.body {
            ElementBody::Normal { children, .. } => {
                let lazy = lazy_block(&children.block(g, Brace::default()));
                let children_ident = Ident::new("children", self.name.span());

                quote!(
                    .#children_ident(#lazy)
                )
            }
            ElementBody::Void { .. } => quote!(),
        };

        let name = &self.name;

        g.push_in_block(Brace::default(), |g| {
            g.push_stmt(quote! { #(#owned)* });
            g.push_expr::<Self::Context>(
                Paren::default(),
                quote! {
                        #name::builder()
                            #(#props)*
                            #children
                            .build()
                },
            );
        });
    }
}

fn lazy_block(block: &AnyBlock) -> TokenStream {
    let buffer_ident = Generator::buffer_ident();
    quote! {
        ::hypertext::Lazy::dangerously_create(
            |#buffer_ident: &mut ::hypertext::Buffer|
            #block
        )
    }
}

pub struct ComponentAttribute(pub Attribute);

impl ComponentAttribute {
    fn kind_expr(&self, g: &mut Generator, owned: &mut Vec<TokenStream>) -> TokenStream {
        let lits = self.0.name.lits();
        let name_str = lits.iter().map(syn::LitStr::value).collect::<String>();
        let span = lits.first().map_or_else(Span::call_site, Spanned::span);
        let name = Ident::new(&name_str, span);
        let maybe_name = format_ident!("maybe_{name}", span = span);
        match &self.0.kind {
            AttributeKind::Value { value, toggle } => {
                let value_expr = Self::value_expr(value, g, owned);
                toggle.as_ref().map_or_else(
                    || quote!(.#name(#value_expr)),
                    |toggle| {
                        let toggle_expr = &toggle.expr;
                        quote!(.#maybe_name(if #toggle_expr { Some(#value_expr) } else { None }))
                    },
                )
            }
            AttributeKind::Option(toggle) => {
                let toggle_expr = &toggle.expr;
                quote!(.#maybe_name(#toggle_expr))
            }
            AttributeKind::Empty(None) => {
                quote!(.#name(true))
            }
            AttributeKind::Empty(Some(toggle)) => {
                let toggle_expr = &toggle.expr;
                quote!(.#maybe_name(if #toggle_expr { Some(true) } else { None }))
            }
            AttributeKind::ClassList(classes) => {
                let to_litstr = |class: &Class| {
                    let Class::Value {
                        value,
                        toggle: None,
                    } = class
                    else {
                        return None;
                    };

                    let AttributeValue::Literal(literal) = value else {
                        return None;
                    };

                    Some(literal.lit_str())
                };

                let to_class = |static_str: &str| {
                    let literal = Literal::Str(LitStr::new(static_str, span));
                    Class::Value {
                        value: AttributeValue::Literal(literal),
                        toggle: None,
                    }
                };

                let mut current_static: String = String::new();
                let mut class_exprs: Vec<TokenStream> = vec![];

                // Collect consecutive static classes into a single string
                for class in classes {
                    if let Some(stat) = to_litstr(class) {
                        if !current_static.is_empty() {
                            current_static.push(' ');
                        }
                        current_static.push_str(&stat.value());
                    } else {
                        let static_class = to_class(&current_static);
                        class_exprs.push(Self::class_expr(&static_class, g, owned));
                        class_exprs.push(Self::class_expr(class, g, owned));
                        current_static.clear();
                    }
                }

                // If no expressions were pushed, the whole class expression is just a literal:
                if class_exprs.is_empty() {
                    let literal_class = Literal::Str(LitStr::new(&current_static, span));
                    quote!(.class(#literal_class))
                } else {
                    if !current_static.is_empty() {
                        let static_class = to_class(&current_static);
                        class_exprs.push(Self::class_expr(&static_class, g, owned));
                    }

                    let ident = Self::push_owned(
                        owned,
                        &quote! {
                            [#(#class_exprs),*]
                                .into_iter()
                                .flatten()
                                .fold(String::new(), |mut classes, class| {
                                    if !classes.is_empty() { classes.push(' '); }
                                    classes.push_str(class);
                                    classes
                                });
                        },
                        span,
                    );

                    quote!(.class(&#ident))
                }
            }
        }
    }

    fn value_expr(
        value: &AttributeValue,
        g: &mut Generator,
        owned: &mut Vec<TokenStream>,
    ) -> TokenStream {
        match value {
            AttributeValue::Literal(literal) => literal.into_token_stream(),
            AttributeValue::Group(group) => {
                let block = g.block_with(Brace::default(), |g| g.push_all(&group.0.0));
                let lazy = lazy_block(&block);
                let ident =
                    Self::push_owned(owned, &quote! { #lazy.render().into_inner() }, block.span());
                quote!(&(#ident))
            }
            AttributeValue::Control(control) => {
                let block = g.block_with(Brace::default(), |g| {
                    g.push(control);
                });
                quote! { #block }
            }
            AttributeValue::Expr(paren_expr) => {
                let expr_tokens = paren_expr.to_token_stream();
                quote!(#[allow(unused_parens)] #expr_tokens)
            }
            AttributeValue::DisplayExpr(expr) => {
                let expr_tokens = expr.wrapped_expr();
                quote! { #expr_tokens }
            }
            AttributeValue::DebugExpr(expr) => {
                let expr_tokens = expr.wrapped_expr();
                quote! { #expr_tokens }
            }
            AttributeValue::Ident(ident) => quote! { #ident },
        }
    }

    fn class_expr(class: &Class, g: &mut Generator, owned: &mut Vec<TokenStream>) -> TokenStream {
        match class {
            Class::Value {
                value,
                toggle: None,
            } => {
                let value_expr = Self::value_expr(value, g, owned);
                quote! { Some(#value_expr) }
            }
            Class::Value {
                value,
                toggle: Some(toggle),
            } => {
                let value_expr = Self::value_expr(value, g, owned);
                let toggle_expr = &toggle.expr;
                quote! { if #toggle_expr { Some(#value_expr) } else { None } }
            }
            Class::Option(toggle) => {
                let toggle_expr = &toggle.expr;
                quote! { #toggle_expr }
            }
        }
    }

    fn push_owned(alloc: &mut Vec<TokenStream>, expr: &TokenStream, span: Span) -> Ident {
        let ident = format_ident!("owned_{}", alloc.len(), span = span);
        alloc.push(quote!(let #ident = #expr;));
        ident
    }
}

impl Parse for ComponentAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
