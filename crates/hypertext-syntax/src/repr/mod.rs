#![expect(clippy::large_enum_variant)]

mod attributes;
mod basics;
mod component;
mod control;
pub mod kw;

use std::{borrow::Cow, marker::PhantomData};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    LitStr, Token, braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
};

pub use self::{attributes::*, basics::*, component::*, control::*};
use crate::{AnyBlock, Context, ElementCheck, ElementKind, Generate, Generator, syntaxes::Syntax};

pub type Document<S> = Many<Node<S>>;

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
    pub lt_token: Token![<],
    pub bang_token: Token![!],
    pub doctype_token: kw::DOCTYPE,
    pub html_token: kw::html,
    pub gt_token: Token![>],
    pub phantom: PhantomData<S>,
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
    pub paren_token: Paren,
    pub expr: TokenStream,
    pub phantom: PhantomData<C>,
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
    pub percent_token: Token![%],
    pub paren_expr: ParenExpr<C>,
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
    pub question_token: Token![?],
    pub expr: ParenExpr<C>,
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

pub struct Group<C: Context>(pub Many<C>);

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

pub struct Many<C: Context>(pub Vec<C>);

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
    pub name: UnquotedName,
    pub attrs: Vec<Attribute>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Element<S> {}

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
                    el_checks.set_closing_tag(closing_name);
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
