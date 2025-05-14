#![expect(clippy::struct_field_names)]

use std::{
    fmt::{self, Display, Formatter},
    marker::PhantomData,
};

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Expr, Ident, LitBool, LitInt, LitStr, Local, Pat, Stmt, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    token::{Brace, Token},
};

use crate::generate::{AnyBlock, ElementChecks, ElementKind, Generate, Generator};

pub trait Syntax {
    type NodeSeparator: Parse;
}

pub trait ControlSyntax: Syntax {
    type ControlToken: Token + Parse;
}

pub struct Markup<S: Syntax> {
    pub doctype: Option<Span>,
    pub nodes: Nodes<S, ElementNode<S>>,
}

impl<S: Syntax> Generate for Markup<S> {
    fn generate(&self, g: &mut Generator) {
        if let Some(doctype) = self.doctype {
            g.push_spanned_str("<!DOCTYPE html>", doctype);
        }

        g.push(&self.nodes);
    }
}

pub trait Node<S: Syntax>: Generate + Sized {
    type Child: Node<S>;

    fn is_control(&self) -> bool;

    fn as_group(&self) -> Option<&Group<S, Self::Child>>;

    fn in_block(&self, g: &mut Generator) -> AnyBlock {
        if let Some(group) = self.as_group() {
            group.block(g)
        } else {
            g.block_with(Brace::default(), |g| g.push(self))
        }
    }
}

pub enum ElementNode<S: Syntax> {
    Element(Element<S>),
    Component(Component<S>),
    Literal(Literal),
    Control(Control<S, Self>),
    Expr(AnyExpr<S>),
    Group(Group<S, Self>),
}

impl<S: Syntax> Node<S> for ElementNode<S> {
    type Child = Self;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for ElementNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Element(element) => g.push(element),
            Self::Component(component) => g.push(component),
            Self::Literal(lit) => g.push_text_lit(&lit.lit_str()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_text(g),
            Self::Group(group) => g.push(group),
        }
    }
}

pub struct AnyExpr<S: Syntax> {
    pub expr: TokenStream,
    pub phantom: PhantomData<S>,
}

impl<S: Syntax> AnyExpr<S> {
    fn generate_text(&self, g: &mut Generator) {
        g.push_text_expr(&self.expr);
    }

    fn generate_attribute(&self, g: &mut Generator) {
        g.push_attribute_expr(&self.expr);
    }
}

pub struct Group<S: Syntax, N: Node<S>>(pub Nodes<S, N>);

impl<S: Syntax, N: Node<S>> Group<S, N> {
    fn block(&self, g: &mut Generator) -> AnyBlock {
        self.0.block(g, Brace::default())
    }
}

impl<S: Syntax, N: Node<S>> Generate for Group<S, N> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.0);
    }
}

pub struct Nodes<S: Syntax, N: Node<S>> {
    pub nodes: Vec<N>,
    pub phantom: PhantomData<S>,
}

impl<S: Syntax, N: Node<S>> Nodes<S, N> {
    fn block(&self, g: &mut Generator, brace_token: Brace) -> AnyBlock {
        g.block_with(brace_token, |g| {
            g.push_all(&self.nodes);
        })
    }
}

impl<S: Syntax, N: Node<S> + Parse> Parse for Nodes<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            nodes: input
                .call(Punctuated::<N, S::NodeSeparator>::parse_terminated)?
                .into_iter()
                .collect(),
            phantom: PhantomData,
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for Nodes<S, N> {
    fn generate(&self, g: &mut Generator) {
        if self.nodes.iter().any(Node::is_control) {
            g.push_in_block(Brace::default(), |g| g.push_all(&self.nodes));
        } else {
            g.push_all(&self.nodes);
        }
    }
}

pub struct Element<S: Syntax> {
    pub name: UnquotedName,
    pub attrs: Vec<Attribute<S>>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Element<S> {
    fn generate(&self, g: &mut Generator) {
        let mut el_checks = ElementChecks::new(&self.name, self.body.kind());

        g.push_str("<");
        g.push_unquoted_name(&self.name);

        for attr in &self.attrs {
            g.push(attr);

            if !attr.name.is_data() {
                if let Some(namespace) = attr.name.namespace() {
                    el_checks.push_namespace(&namespace);
                } else {
                    el_checks.push_attribute(&attr.name);
                }
            }
        }

        g.push_str(">");

        match &self.body {
            ElementBody::Normal {
                children,
                closing_name,
            } => {
                let name = closing_name.as_ref().map_or(&self.name, |closing_name| {
                    el_checks.set_closing_name(closing_name);
                    closing_name
                });

                g.push(children);
                g.push_str("</");
                g.push_unquoted_name(name);
                g.push_str(">");
            }
            ElementBody::Void => {}
        }

        g.record_element(el_checks);
    }
}

pub enum ElementBody<S: Syntax> {
    Normal {
        children: Nodes<S, ElementNode<S>>,
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

pub struct Attribute<S: Syntax> {
    pub name: UnquotedName,
    pub kind: AttributeKind<S>,
}

impl<S: Syntax> Generate for Attribute<S> {
    fn generate(&self, g: &mut Generator) {
        match &self.kind {
            AttributeKind::Value { value, toggle, .. } => {
                if let Some(toggle) = toggle {
                    g.push_conditional(toggle.parenthesized(), |g| {
                        g.push_str(" ");
                        g.push_unquoted_name(&self.name);
                        g.push_str("=\"");
                        g.push(value);
                        g.push_str("\"");
                    });
                } else {
                    g.push_str(" ");
                    g.push_unquoted_name(&self.name);
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
                        g.push_unquoted_name(&self.name);
                        g.push_str("=\"");
                        g.push_attribute_expr(&value);
                        g.push_str("\"");
                    },
                );
            }
            AttributeKind::Empty(Some(toggle)) => {
                g.push_conditional(toggle.parenthesized(), |g| {
                    g.push_str(" ");
                    g.push_unquoted_name(&self.name);
                });
            }
            AttributeKind::Empty(None) => {
                g.push_str(" ");
                g.push_unquoted_name(&self.name);
            }
            AttributeKind::ClassList(classes) => {
                g.push_str(" ");
                g.push_unquoted_name(&self.name);
                g.push_str("=\"");

                for (i, class) in classes.iter().enumerate() {
                    if let Some(toggle) = &class.toggle {
                        g.push_conditional(toggle.parenthesized(), |g| {
                            if i > 0 {
                                g.push_str(" ");
                            }

                            g.push(&class.value);
                        });
                    } else {
                        if i > 0 {
                            g.push_str(" ");
                        }

                        g.push(&class.value);
                    }
                }

                g.push_str("\"");
            }
        }
    }
}

pub enum AttributeKind<S: Syntax> {
    Value {
        value: UnquotedValueNode<S>,
        toggle: Option<Toggle<S>>,
    },
    Empty(Option<Toggle<S>>),
    Option(Toggle<S>),
    ClassList(Vec<Class<S>>),
}

pub struct Class<S: Syntax> {
    pub value: UnquotedValueNode<S>,
    pub toggle: Option<Toggle<S>>,
}

pub enum UnquotedValueNode<S: Syntax> {
    UnquotedName(UnquotedName),
    Literal(LitStr),
    Group(Group<S, QuotedValueNode<S>>),
    Control(Control<S, QuotedValueNode<S>>),
    Expr(AnyExpr<S>),
}

impl<S: Syntax> Node<S> for UnquotedValueNode<S> {
    type Child = QuotedValueNode<S>;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for UnquotedValueNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::UnquotedName(name) => g.push_unquoted_name(name),
            Self::Group(block) => g.push(block),
            Self::Literal(lit) => g.push_attribute_lit(&lit.clone()),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_attribute(g),
        }
    }
}

pub enum QuotedValueNode<S: Syntax> {
    Literal(Literal),
    Group(Group<S, Self>),
    Control(Control<S, Self>),
    Expr(AnyExpr<S>),
}

impl<S: Syntax> Node<S> for QuotedValueNode<S> {
    type Child = Self;

    fn is_control(&self) -> bool {
        matches!(self, Self::Control(_))
    }

    fn as_group(&self) -> Option<&Group<S, Self::Child>> {
        match self {
            Self::Group(group) => Some(group),
            _ => None,
        }
    }
}

impl<S: Syntax> Generate for QuotedValueNode<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Literal(lit) => g.push_attribute_lit(&lit.lit_str()),
            Self::Group(block) => g.push(block),
            Self::Control(control) => g.push(control),
            Self::Expr(expr) => expr.generate_attribute(g),
        }
    }
}

pub struct Toggle<S: Syntax> {
    pub expr: TokenStream,
    pub phantom: PhantomData<S>,
}

impl<S: Syntax> Toggle<S> {
    fn parenthesized(&self) -> TokenStream {
        let expr = &self.expr;

        quote!((#expr))
    }
}

pub struct Component<S: Syntax> {
    pub name: Ident,
    pub attrs: Vec<ComponentAttribute<S>>,
    pub body: ElementBody<S>,
}

impl<S: Syntax> Generate for Component<S> {
    fn generate(&self, g: &mut Generator) {
        let fields = self.attrs.iter().map(|attr| {
            let name = &attr.name;
            let expr = &attr.value.expr();
            let value = quote_spanned!(expr.span()=> (#expr));

            quote!(#name: #value)
        });

        let children = match &self.body {
            ElementBody::Normal { children, .. } => {
                let output_ident = Ident::new("hypertext_output", Span::mixed_site());

                let block = g.block_with(Brace::default(), |g| {
                    g.push(children);
                });

                let lazy = quote! {
                    ::hypertext::Lazy({
                        extern crate alloc;

                        move |#output_ident: &mut alloc::string::String|
                            #block
                    })
                };

                quote!(
                    children: #lazy,
                )
            }
            ElementBody::Void => quote!(),
        };

        let name = &self.name;

        let init = quote! {
            #name {
                #(#fields,)*
                #children
            }
        };

        g.push_text_expr(&init);
    }
}

pub struct ComponentAttribute<S: Syntax> {
    pub name: Ident,
    pub value: ComponentAttributeValue<S>,
}

pub enum ComponentAttributeValue<S: Syntax> {
    UnquotedName(UnquotedName),
    Literal(Literal),
    Expr(AnyExpr<S>),
}

impl<S: Syntax> ComponentAttributeValue<S> {
    fn expr(&self) -> TokenStream {
        match self {
            Self::UnquotedName(name) => {
                let strs = name.strs();

                quote!(::core::concat!(#(#strs),*))
            }
            Self::Literal(lit) => match lit {
                Literal::Str(lit) => lit.to_token_stream(),
                Literal::Int(lit) => lit.to_token_stream(),
                Literal::Bool(lit) => lit.to_token_stream(),
            },
            Self::Expr(expr) => expr.expr.clone(),
        }
    }
}

impl<S: Syntax> Generate for ComponentAttributeValue<S> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::UnquotedName(name) => g.push_unquoted_name(name),
            Self::Literal(lit) => g.push_attribute_lit(&lit.lit_str()),
            Self::Expr(expr) => expr.generate_attribute(g),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct UnquotedName(pub Punctuated<NameFragment, NamePunct>);

impl UnquotedName {
    fn is_data(&self) -> bool {
        self.0.pairs().next().is_some_and(|pair| {
            matches!(pair, Pair::Punctuated(NameFragment::Ident(ident), NamePunct::Hyphen(_)) if ident == "data")
        })
    }

    pub fn is_component(&self) -> bool {
        let mut pairs = self.0.pairs().map(Pair::into_tuple);

        matches!(
            (pairs.next(), pairs.next()),
            (Some((NameFragment::Ident(ident), None)), None)
            if ident
                .to_string()
                .chars()
                .next()
                .is_some_and(char::is_uppercase)
        )
    }

    fn namespace(&self) -> Option<Self> {
        let mut punctuated = Punctuated::new();

        for (fragment, punct) in self.0.pairs().map(Pair::into_tuple) {
            punctuated.push_value(fragment.clone());

            if let Some(punct) = punct {
                match punct {
                    NamePunct::Colon(_) => return Some(Self(punctuated)),
                    NamePunct::Hyphen(_) => punctuated.push_punct(*punct),
                }
            }
        }

        None
    }

    pub fn spans(&self) -> Vec<Span> {
        let mut spans = Vec::new();

        for (fragment, punct) in self.0.pairs().map(Pair::into_tuple) {
            match fragment {
                NameFragment::Ident(ident) => spans.push(ident.span()),
                NameFragment::Number(num) => spans.push(num.span()),
                NameFragment::Empty => {}
            }

            if let Some(punct) = punct {
                spans.push(punct.span());
            }
        }

        spans
    }

    pub fn strs(&self) -> Vec<LitStr> {
        let mut strs = Vec::new();

        for (fragment, punct) in self.0.pairs().map(Pair::into_tuple) {
            match fragment {
                NameFragment::Ident(ident) => {
                    strs.push(LitStr::new(&ident.to_string(), ident.span()));
                }
                NameFragment::Number(num) => {
                    strs.push(LitStr::new(&num.to_string(), num.span()));
                }
                NameFragment::Empty => {}
            }

            if let Some(punct) = punct {
                strs.push(LitStr::new(&punct.to_char().to_string(), punct.span()));
            }
        }

        strs
    }

    pub fn ident_string(&self) -> String {
        let mut s = String::new();

        for (fragment, punct) in self.0.pairs().map(Pair::into_tuple) {
            match fragment {
                NameFragment::Ident(ident) => {
                    s.push_str(&ident.to_string());
                }
                NameFragment::Number(num) => {
                    s.push_str(&num.to_string());
                }
                NameFragment::Empty => {}
            }

            if punct.is_some() {
                s.push('_');
            }
        }

        if s == "super" || s == "self" || s == "Self" || s == "extern" || s == "crate" || s == "_" {
            s.insert(0, '_');
        }

        s
    }
}

impl Parse for UnquotedName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self({
            let mut name = Punctuated::new();

            loop {
                name.push_value(input.parse()?);

                if !(input.peek(Token![-]) || input.peek(Token![:])) {
                    break;
                }

                name.push_punct(input.parse()?);
            }

            name
        }))
    }
}

impl Display for UnquotedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (fragment, punct) in self.0.pairs().map(Pair::into_tuple) {
            match fragment {
                NameFragment::Ident(ident) => {
                    write!(f, "{ident}")?;
                }
                NameFragment::Number(num) => {
                    write!(f, "{num}")?;
                }
                NameFragment::Empty => {}
            }

            if let Some(punct) = punct {
                write!(f, "{}", punct.to_char())?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum NameFragment {
    Ident(Ident),
    Number(LitInt),
    Empty,
}

impl Parse for NameFragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Ident)
        } else if input.peek(LitInt) {
            let int = input.parse::<LitInt>()?;

            if !int.suffix().is_empty() {
                return Err(syn::Error::new_spanned(
                    &int,
                    "integer suffixes are not allowed in names",
                ));
            }

            Ok(Self::Number(int))
        } else {
            Ok(Self::Empty)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NamePunct {
    Colon(Token![:]),
    Hyphen(Token![-]),
}

impl NamePunct {
    const fn to_char(self) -> char {
        match self {
            Self::Colon(_) => ':',
            Self::Hyphen(_) => '-',
        }
    }

    fn span(self) -> Span {
        match self {
            Self::Colon(colon) => colon.span(),
            Self::Hyphen(hyphen) => hyphen.span(),
        }
    }
}

impl Parse for NamePunct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else if lookahead.peek(Token![-]) {
            input.parse().map(Self::Hyphen)
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum Literal {
    Str(LitStr),
    Int(LitInt),
    Bool(LitBool),
}

impl Literal {
    fn lit_str(&self) -> LitStr {
        match self {
            Self::Str(lit) => lit.clone(),
            Self::Int(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Bool(lit) => LitStr::new(&lit.value.to_string(), lit.span()),
        }
    }
}

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            input.parse().map(Self::Str)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else {
            Err(lookahead.error())
        }
    }
}

pub enum Control<S: Syntax, N: Node<S>> {
    Let(Let),
    If(If<S, N>),
    For(For<S, N>),
    While(While<S, N>),
    Match(Match<S, N>),
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for Control<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<S::ControlToken>()?;

        let lookahead = input.lookahead1();

        if lookahead.peek(Token![let]) {
            input.parse().map(Self::Let)
        } else if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Token![for]) {
            input.parse().map(Self::For)
        } else if lookahead.peek(Token![while]) {
            input.parse().map(Self::While)
        } else if lookahead.peek(Token![match]) {
            input.parse().map(Self::Match)
        } else {
            Err(lookahead.error())
        }
    }
}

impl<S: Syntax, N: Node<S>> Generate for Control<S, N> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Let(let_) => g.push(let_),
            Self::If(if_) => g.push(if_),
            Self::For(for_) => g.push(for_),
            Self::While(while_) => g.push(while_),
            Self::Match(match_) => g.push(match_),
        }
    }
}

pub struct Let(pub Local);

impl Parse for Let {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let local = match input.parse()? {
            Stmt::Local(local) => local,
            stmt => return Err(syn::Error::new_spanned(stmt, "expected `let` statement")),
        };

        Ok(Self(local))
    }
}

impl Generate for Let {
    fn generate(&self, g: &mut Generator) {
        g.push_stmt(&self.0);
    }
}

pub struct ControlBlock<S: Syntax, N: Node<S>> {
    pub brace_token: Brace,
    pub nodes: Nodes<S, N>,
}

impl<S: Syntax, N: Node<S>> ControlBlock<S, N> {
    fn block(&self, g: &mut Generator) -> AnyBlock {
        self.nodes.block(g, self.brace_token)
    }
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for ControlBlock<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            brace_token: braced!(content in input),
            nodes: content.parse()?,
        })
    }
}

pub struct If<S: Syntax, N: Node<S>> {
    if_token: Token![if],
    cond: Expr,
    then_block: ControlBlock<S, N>,
    else_branch: Option<(Token![else], Box<ControlIfOrBlock<S, N>>)>,
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for If<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            then_block: input.parse()?,
            else_branch: if S::ControlToken::peek(input.cursor()) && input.peek2(Token![else]) {
                input.parse::<S::ControlToken>()?;

                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for If<S, N> {
    fn generate(&self, g: &mut Generator) {
        fn to_expr<S: Syntax, N: Node<S>>(if_: &If<S, N>, g: &mut Generator) -> TokenStream {
            let if_token = if_.if_token;
            let cond = &if_.cond;
            let then_block = if_.then_block.block(g);
            let else_branch = if_.else_branch.as_ref().map(|(else_token, if_or_block)| {
                let else_block = match &**if_or_block {
                    ControlIfOrBlock::If(if_) => to_expr(if_, g),
                    ControlIfOrBlock::Block(block) => block.block(g).to_token_stream(),
                };

                quote! {
                    #else_token #else_block
                }
            });

            quote! {
                #if_token #cond
                    #then_block
                #else_branch
            }
        }

        let expr = to_expr(self, g);

        g.push_stmt(expr);
    }
}

pub enum ControlIfOrBlock<S: Syntax, N: Node<S>> {
    If(If<S, N>),
    Block(ControlBlock<S, N>),
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for ControlIfOrBlock<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![if]) {
            input.parse().map(Self::If)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct For<S: Syntax, N: Node<S>> {
    pub for_token: Token![for],
    pub pat: Pat,
    pub in_token: Token![in],
    pub expr: Expr,
    pub block: ControlBlock<S, N>,
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for For<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            in_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            block: input.parse()?,
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for For<S, N> {
    fn generate(&self, g: &mut Generator) {
        let for_token = self.for_token;
        let pat = &self.pat;
        let in_token = self.in_token;
        let expr = &self.expr;
        let block = self.block.block(g);

        g.push_stmt(quote! {
            #for_token #pat #in_token #expr
                #block
        });
    }
}

pub struct While<S: Syntax, N: Node<S>> {
    pub while_token: Token![while],
    pub cond: Expr,
    pub block: ControlBlock<S, N>,
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for While<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            while_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            block: input.parse()?,
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for While<S, N> {
    fn generate(&self, g: &mut Generator) {
        let while_token = self.while_token;
        let cond = &self.cond;
        let block = self.block.block(g);

        g.push_stmt(quote! {
            #while_token #cond
                #block
        });
    }
}

pub struct Match<S: Syntax, N: Node<S>> {
    match_token: Token![match],
    expr: Expr,
    brace_token: Brace,
    arms: Vec<MatchNodeArm<S, N>>,
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for Match<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(Self {
            match_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            brace_token: braced!(content in input),
            arms: {
                let mut arms = Vec::new();

                while !content.is_empty() {
                    arms.push(content.parse()?);
                }

                arms
            },
        })
    }
}

impl<S: Syntax, N: Node<S>> Generate for Match<S, N> {
    fn generate(&self, g: &mut Generator) {
        let arms = self
            .arms
            .iter()
            .map(|arm| {
                let pat = arm.pat.clone();
                let guard = arm
                    .guard
                    .as_ref()
                    .map(|(if_token, guard)| quote!(#if_token #guard));
                let fat_arrow_token = arm.fat_arrow_token;
                let block = match &arm.body {
                    MatchNodeArmBody::Block(block) => block.block(g),
                    MatchNodeArmBody::Node(node) => node.in_block(g),
                };
                let comma = arm.comma_token;

                quote!(#pat #guard #fat_arrow_token #block #comma)
            })
            .collect::<TokenStream>();

        let match_token = self.match_token;
        let expr = &self.expr;

        let mut stmt = quote!(#match_token #expr);

        self.brace_token
            .surround(&mut stmt, |tokens| tokens.extend(arms));

        g.push_stmt(stmt);
    }
}

pub struct MatchNodeArm<S: Syntax, N: Node<S>> {
    pub pat: Pat,
    pub guard: Option<(Token![if], Expr)>,
    pub fat_arrow_token: Token![=>],
    pub body: MatchNodeArmBody<S, N>,
    pub comma_token: Option<Token![,]>,
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for MatchNodeArm<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            guard: if input.peek(Token![if]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            },
            fat_arrow_token: input.parse()?,
            body: input.parse()?,
            comma_token: input.parse()?,
        })
    }
}

pub enum MatchNodeArmBody<S: Syntax, N: Node<S>> {
    Block(ControlBlock<S, N>),
    Node(N),
}

impl<S: ControlSyntax, N: Node<S> + Parse> Parse for MatchNodeArmBody<S, N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            input.parse().map(Self::Node)
        }
    }
}
