#![allow(clippy::struct_field_names)]

use std::ops::ControlFlow;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Arm, Expr, ExprBlock, ExprForLoop, ExprIf, ExprMatch, ExprParen, ExprWhile, Ident, LitBool,
    LitInt, LitStr, Local, Pat, Stmt, Token, braced, bracketed,
    ext::IdentExt,
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    token::{At, Brace, Bracket, Comma, Else, FatArrow, For, If, In, Match, Paren, While},
};

use crate::generate::{Generate, Generator};

pub fn parse(tokens: TokenStream) -> syn::Result<Markup> {
    syn::parse2(tokens)
}

#[derive(Debug, Clone)]
pub struct Markup {
    doctype: Option<Doctype>,
    nodes: Vec<ElementNode>,
}

impl Parse for Markup {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            doctype: if input.peek(Token![!]) && input.peek2(DOCTYPE) {
                Some(input.parse()?)
            } else {
                None
            },
            nodes: {
                let mut nodes = Vec::new();
                while !input.is_empty() {
                    nodes.push(input.parse()?);
                }
                nodes
            },
        })
    }
}

impl ToTokens for Markup {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for node in &self.nodes {
            node.to_tokens(tokens);
        }
    }
}

impl Generate for Markup {
    fn generate(&self, g: &mut Generator) {
        if let Some(doctype) = &self.doctype {
            g.push(doctype);
        }

        g.push_all(&self.nodes);
    }
}

syn::custom_keyword!(DOCTYPE);

#[derive(Debug, Clone)]
struct Doctype {
    bang_token: Token![!],
    name: DOCTYPE,
}

impl Parse for Doctype {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            bang_token: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ToTokens for Doctype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bang_token.to_tokens(tokens);
        self.name.to_tokens(tokens);
    }
}

impl Generate for Doctype {
    fn generate(&self, g: &mut Generator) {
        g.push_spanned_str("<!DOCTYPE html>", self.span());
    }
}

trait Node: Parse + ToTokens + Generate {
    fn is_let(&self) -> bool;
}

#[derive(Debug, Clone)]
enum ElementNode {
    Block(Block<Self>),
    Element(Element),
    Splice(Splice),
    Literal(Lit),
    Keyword(Keyword<Self>),
}

impl Node for ElementNode {
    fn is_let(&self) -> bool {
        matches!(
            self,
            Self::Keyword(Keyword {
                kind: KeywordKind::Let(_),
                ..
            })
        )
    }
}

impl Parse for ElementNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Splice)
        } else if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Element)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Keyword)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ElementNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Block(block) => block.to_tokens(tokens),
            Self::Element(element) => element.to_tokens(tokens),
            Self::Splice(splice) => splice.to_tokens(tokens),
            Self::Literal(lit) => lit.to_tokens(tokens),
            Self::Keyword(kw) => kw.to_tokens(tokens),
        }
    }
}

impl Generate for ElementNode {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Block(block) => g.push(block),
            Self::Element(element) => g.push(element),
            Self::Splice(splice) => g.push(splice),
            Self::Literal(lit) => g.push(lit),
            Self::Keyword(kw) => g.push(kw),
        }
    }
}

#[derive(Debug, Clone)]
struct Block<N> {
    brace_token: Brace,
    nodes: Vec<N>,
}

impl<N: Node> Parse for Block<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace_token: braced!(content in input),
            nodes: {
                let mut nodes = Vec::new();
                while !content.is_empty() {
                    nodes.push(content.parse()?);
                }
                nodes
            },
        })
    }
}

impl<N: Node> ToTokens for Block<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            for node in &self.nodes {
                node.to_tokens(tokens);
            }
        });
    }
}

impl<N: Node> Generate for Block<N> {
    fn generate(&self, g: &mut Generator) {
        if self.nodes.iter().any(Node::is_let) {
            g.in_block(|g| g.push_all(&self.nodes));
        } else {
            g.push_all(&self.nodes);
        }
    }
}

#[derive(Debug, Clone)]
struct Splice {
    paren_token: Paren,
    expr: Expr,
}

impl Parse for Splice {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            paren_token: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }
}

impl ToTokens for Splice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}

impl Generate for Splice {
    fn generate(&self, g: &mut Generator) {
        g.push_rendered_expr(&self.expr);
    }
}

#[derive(Debug, Clone)]
struct Element {
    name: Name,
    id: Option<IdAttribute>,
    classes: Option<Classes>,
    attrs: Vec<Attribute>,
    body: ElementBody,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            id: if input.peek(Token![#]) {
                Some(input.parse()?)
            } else {
                None
            },
            classes: if input.peek(Token![.]) {
                Some(input.parse()?)
            } else {
                None
            },
            attrs: {
                let mut attrs = Vec::new();

                while input.peek(Ident::peek_any) || input.peek(LitStr) || input.peek(LitInt) {
                    attrs.push(input.parse()?);
                }

                attrs
            },
            body: input.parse()?,
        })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        if let Some(id) = &self.id {
            id.to_tokens(tokens);
        }
        if let Some(classes) = &self.classes {
            classes.to_tokens(tokens);
        }
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.body.to_tokens(tokens);
    }
}

impl Generate for Element {
    fn generate(&self, g: &mut Generator) {
        g.record_element(&self.name.ident());

        g.push_str("<");
        g.push_escaped_lit(self.name.lit());

        if let Some(id) = &self.id {
            g.record_attribute(&self.name.ident(), &id.attr_name_ident());

            g.push_str(" ");
            g.push(id);
        }

        if let Some(classes) = &self.classes {
            g.record_attribute(&self.name.ident(), &classes.attr_name_ident());

            g.push_str(" ");
            g.push(classes);
        }

        for attr in &self.attrs {
            g.push(attr);

            let mut name_pairs = attr.name.name.pairs();
            let is_data = name_pairs.next().is_some_and(|pair| {
                if let Pair::Punctuated(NameFragment::Ident(ident), NamePunct::Hyphen(_)) = pair {
                    ident == "data"
                } else {
                    false
                }
            }) && name_pairs.next().is_some();

            if !is_data {
                let (attr_ident, is_namespace) = attr.name.ident_or_namespace();

                if is_namespace {
                    g.record_namespace(&self.name.ident(), &attr_ident);
                } else {
                    g.record_attribute(&self.name.ident(), &attr_ident);
                }
            }
        }

        g.push_str(">");

        match &self.body {
            ElementBody::Void(_) => g.record_void_element(&self.name.ident()),
            ElementBody::Block(block) => {
                g.push(block);
                g.push_str("</");
                g.push_escaped_lit(self.name.lit());
                g.push_str(">");
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ElementBody {
    Void(Token![;]),
    Block(Block<ElementNode>),
}

impl Parse for ElementBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![;]) {
            input.parse().map(Self::Void)
        } else if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for ElementBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Void(semi) => semi.to_tokens(tokens),
            Self::Block(block) => block.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone)]
struct IdAttribute {
    pound_token: Token![#],
    value: IdOrClassNode,
}

impl IdAttribute {
    fn attr_name_ident(&self) -> Ident {
        Ident::new("id", self.span())
    }

    fn attr_name_lit(&self) -> LitStr {
        LitStr::new("id", self.span())
    }
}

impl Parse for IdAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pound_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for IdAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

impl Generate for IdAttribute {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(self.attr_name_lit());
        g.push_str("=\"");
        g.push(&self.value);
        g.push_str("\"");
    }
}

#[derive(Debug, Clone)]
struct Classes {
    classes: Vec<Class>,
    toggled_classes: Vec<ToggledClass>,
}

impl Classes {
    fn attr_name_ident(&self) -> Ident {
        Ident::new("class", self.span())
    }

    fn attr_name_lit(&self) -> LitStr {
        LitStr::new("class", self.span())
    }
}

impl Parse for Classes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut classes = Vec::new();
        let mut toggled_classes = Vec::new();

        loop {
            if !input.peek(Token![.]) {
                break;
            }

            let class = input.parse::<Class>()?;

            if input.peek(Bracket) {
                toggled_classes.push(class.into_toggled(input.parse()?));
                break;
            }

            classes.push(class);
        }

        loop {
            if !input.peek(Token![.]) {
                break;
            }

            toggled_classes.push(input.parse()?);
        }

        Ok(Self {
            classes,
            toggled_classes,
        })
    }
}

impl ToTokens for Classes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for class in &self.classes {
            class.to_tokens(tokens);
        }
        for toggled_class in &self.toggled_classes {
            toggled_class.to_tokens(tokens);
        }
    }
}

impl Generate for Classes {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(self.attr_name_lit());
        g.push_str("=\"");

        for (i, class) in self.classes.iter().enumerate() {
            if i > 0 {
                g.push_str(" ");
            }

            g.push(&class.value);
        }

        for (i, class) in self.toggled_classes.iter().enumerate() {
            g.push_conditional(&class.toggle.parenthesized_cond(), |g| {
                if !self.classes.is_empty() || i > 0 {
                    g.push_str(" ");
                }

                g.push(&class.value);
            });
        }

        g.push_str("\"");
    }
}

#[derive(Debug, Clone)]
struct Class {
    dot_token: Token![.],
    value: IdOrClassNode,
}

impl Class {
    fn into_toggled(self, toggle: Toggle) -> ToggledClass {
        ToggledClass {
            dot_token: self.dot_token,
            value: self.value,
            toggle,
        }
    }
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            dot_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dot_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
struct ToggledClass {
    dot_token: Token![.],
    value: IdOrClassNode,
    toggle: Toggle,
}

impl Parse for ToggledClass {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let class = input.parse::<Class>()?;
        if !input.peek(Bracket) {
            return Err(syn::Error::new_spanned(
                class,
                "normal classes must come before toggled classes",
            ));
        }

        Ok(class.into_toggled(input.parse()?))
    }
}

impl ToTokens for ToggledClass {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dot_token.to_tokens(tokens);
        self.value.to_tokens(tokens);
        self.toggle.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
enum IdOrClassNode {
    Block(Block<Self>),
    Splice(Splice),
    Literal(LitStr),
    Keyword(Keyword<Self>),
    Name(Name),
}

impl Node for IdOrClassNode {
    fn is_let(&self) -> bool {
        matches!(
            self,
            Self::Keyword(Keyword {
                kind: KeywordKind::Let(_),
                ..
            })
        )
    }
}

impl Parse for IdOrClassNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Splice)
        } else if lookahead.peek(LitStr) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Keyword)
        } else if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            input.parse().map(Self::Name)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for IdOrClassNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Block(block) => block.to_tokens(tokens),
            Self::Splice(splice) => splice.to_tokens(tokens),
            Self::Literal(lit) => lit.to_tokens(tokens),
            Self::Keyword(kw) => kw.to_tokens(tokens),
            Self::Name(name) => name.to_tokens(tokens),
        }
    }
}

impl Generate for IdOrClassNode {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Block(block) => g.push(block),
            Self::Splice(splice) => g.push(splice),
            Self::Literal(lit) => g.push_escaped_lit(lit.clone()),
            Self::Keyword(kw) => g.push(kw),
            Self::Name(name) => g.push_escaped_lit(name.lit()),
        }
    }
}

#[derive(Debug, Clone)]
struct Attribute {
    name: Name,
    kind: AttributeKind,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            kind: input.parse()?,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl Generate for Attribute {
    fn generate(&self, g: &mut Generator) {
        match &self.kind {
            AttributeKind::Normal {
                value,
                toggle: Some(toggle),
                ..
            } => g.push_conditional(&toggle.parenthesized_cond(), |g| {
                g.push_str(" ");
                g.push_escaped_lit(self.name.lit());
                g.push_str("=\"");
                g.push(value);
                g.push_str("\"");
            }),
            AttributeKind::Normal {
                value,
                toggle: None,
                ..
            } => {
                g.push_str(" ");
                g.push_escaped_lit(self.name.lit());
                g.push_str("=\"");
                g.push(value);
                g.push_str("\"");
            }
            AttributeKind::Optional {
                toggle: Toggle { cond, .. },
                ..
            } => g.push_conditional(
                &parse_quote!(let ::core::option::Option::Some(value) = (#cond)),
                |g| {
                    g.push_str(" ");
                    g.push_escaped_lit(self.name.lit());
                    g.push_str("=\"");
                    g.push_rendered_expr(&parse_quote!(value));
                    g.push_str("\"");
                },
            ),
            AttributeKind::Empty(Some(toggle)) => {
                g.push_conditional(&toggle.parenthesized_cond(), |g| {
                    g.push_str(" ");
                    g.push_escaped_lit(self.name.lit());
                });
            }
            AttributeKind::Empty(None) => {
                g.push_str(" ");
                g.push_escaped_lit(self.name.lit());
            }
        }
    }
}

#[derive(Debug, Clone)]
enum AttributeKind {
    Normal {
        eq_token: Token![=],
        value: AttributeValueNode,
        toggle: Option<Toggle>,
    },
    Optional {
        eq_token: Token![=],
        toggle: Toggle,
    },
    Empty(Option<Toggle>),
}

impl Parse for AttributeKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![=]) {
            let eq_token = input.parse()?;

            if input.peek(Bracket) {
                Ok(Self::Optional {
                    eq_token,
                    toggle: input.parse()?,
                })
            } else {
                Ok(Self::Normal {
                    eq_token,
                    value: input.parse()?,
                    toggle: if input.peek(Bracket) {
                        Some(input.parse()?)
                    } else {
                        None
                    },
                })
            }
        } else if lookahead.peek(Bracket) {
            Ok(Self::Empty(Some(input.parse()?)))
        } else {
            Ok(Self::Empty(None))
        }
    }
}

impl ToTokens for AttributeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Normal {
                eq_token,
                value,
                toggle,
            } => {
                eq_token.to_tokens(tokens);
                value.to_tokens(tokens);
                if let Some(toggle) = toggle {
                    toggle.to_tokens(tokens);
                }
            }
            Self::Optional { eq_token, toggle } => {
                eq_token.to_tokens(tokens);
                toggle.to_tokens(tokens);
            }
            Self::Empty(toggle) => {
                if let Some(toggle) = toggle {
                    toggle.to_tokens(tokens);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum AttributeValueNode {
    Block(Block<Self>),
    Splice(Splice),
    Literal(Lit),
    Keyword(Keyword<Self>),
}

impl Node for AttributeValueNode {
    fn is_let(&self) -> bool {
        matches!(
            self,
            Self::Keyword(Keyword {
                kind: KeywordKind::Let(_),
                ..
            })
        )
    }
}

impl Parse for AttributeValueNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Paren) {
            input.parse().map(Self::Splice)
        } else if lookahead.peek(LitStr) || lookahead.peek(LitInt) || lookahead.peek(LitBool) {
            input.parse().map(Self::Literal)
        } else if lookahead.peek(Token![@]) {
            input.parse().map(Self::Keyword)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for AttributeValueNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Block(block) => block.to_tokens(tokens),
            Self::Splice(splice) => splice.to_tokens(tokens),
            Self::Literal(lit) => lit.to_tokens(tokens),
            Self::Keyword(kw) => kw.to_tokens(tokens),
        }
    }
}

impl Generate for AttributeValueNode {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Block(block) => g.push(block),
            Self::Splice(splice) => g.push(splice),
            Self::Literal(lit) => g.push_escaped_lit(lit.lit_str()),
            Self::Keyword(kw) => g.push(kw),
        }
    }
}

#[derive(Debug, Clone)]
struct Name {
    name: Punctuated<NameFragment, NamePunct>,
}

impl Name {
    /// If the ident should be a namespace, the boolean is true.
    fn ident_or_namespace(&self) -> (Ident, bool) {
        let string = self.name.pairs().map(Pair::into_tuple).try_fold(
            String::new(),
            |mut acc, (fragment, punct)| {
                acc.push_str(&fragment.to_fragment_string());

                match punct {
                    Some(NamePunct::Colon(_)) => {
                        return ControlFlow::Break(acc);
                    }
                    Some(NamePunct::Hyphen(_)) => {
                        acc.push('_');
                    }
                    None => {}
                }

                ControlFlow::Continue(acc)
            },
        );

        let (string, is_namespace) = match string {
            ControlFlow::Break(string) => (string, true),
            ControlFlow::Continue(string) => (string, false),
        };

        (
            // results in better editor hover-doc support than unconditional `new_raw` usage
            syn::parse_str::<Ident>(&string).map_or_else(
                |_| Ident::new_raw(&string, self.span()),
                |mut ident| {
                    ident.set_span(self.span());
                    ident
                },
            ),
            is_namespace,
        )
    }

    fn ident(&self) -> Ident {
        let string = self.name.pairs().map(Pair::into_tuple).fold(
            String::new(),
            |mut acc, (fragment, punct)| {
                acc.push_str(&fragment.to_fragment_string());

                if punct.is_some() {
                    acc.push('_');
                }

                acc
            },
        );

        // results in better editor hover-doc support than unconditional `new_raw` usage
        syn::parse_str::<Ident>(&string).map_or_else(
            |_| Ident::new_raw(&string, self.span()),
            |mut ident| {
                ident.set_span(self.span());
                ident
            },
        )
    }

    fn lit(&self) -> LitStr {
        let string = self.name.pairs().map(Pair::into_tuple).fold(
            String::new(),
            |mut acc, (fragment, punct)| {
                acc.push_str(&fragment.to_fragment_string());

                if let Some(punct) = punct {
                    acc.push(punct.to_char());
                }

                acc
            },
        );

        LitStr::new(&string, self.span())
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: {
                let mut punctuated = Punctuated::new();

                loop {
                    punctuated.push_value(input.parse()?);

                    if !(input.peek(Token![-]) || input.peek(Token![:])) {
                        break;
                    }

                    let punct = input.parse()?;
                    punctuated.push_punct(punct);
                }

                punctuated
            },
        })
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
enum NameFragment {
    Ident(Ident),
    Number(LitInt),
    Empty,
}

impl NameFragment {
    fn to_fragment_string(&self) -> String {
        match self {
            Self::Ident(ident) => ident.to_string(),
            Self::Number(num) => num.to_string(),
            Self::Empty => String::new(),
        }
    }
}

impl Parse for NameFragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Ident)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Number)
        } else if lookahead.peek(Token![-]) || lookahead.peek(Token![:]) {
            Ok(Self::Empty)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for NameFragment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Number(lit) => lit.to_tokens(tokens),
            Self::Empty => {}
        }
    }
}

#[derive(Debug, Clone)]
enum NamePunct {
    Colon(Token![:]),
    Hyphen(Token![-]),
}

impl NamePunct {
    const fn to_char(&self) -> char {
        match self {
            Self::Colon(_) => ':',
            Self::Hyphen(_) => '-',
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

impl ToTokens for NamePunct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Colon(token) => token.to_tokens(tokens),
            Self::Hyphen(token) => token.to_tokens(tokens),
        }
    }
}

#[derive(Debug, Clone)]
enum Lit {
    Str(LitStr),
    Int(LitInt),
    Bool(LitBool),
}

impl Lit {
    fn lit_str(&self) -> LitStr {
        match self {
            Self::Str(lit) => lit.clone(),
            Self::Int(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Bool(lit) => LitStr::new(&lit.value.to_string(), lit.span()),
        }
    }
}

impl Parse for Lit {
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

impl ToTokens for Lit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Str(lit) => lit.to_tokens(tokens),
            Self::Int(lit) => lit.to_tokens(tokens),
            Self::Bool(lit) => lit.to_tokens(tokens),
        }
    }
}

impl Generate for Lit {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(self.lit_str());
    }
}

#[derive(Debug, Clone)]
struct Toggle {
    bracket_token: Bracket,
    cond: Expr,
}

impl Toggle {
    fn parenthesized_cond(&self) -> Expr {
        Expr::Paren(ExprParen {
            attrs: Vec::new(),
            paren_token: Paren::default(),
            expr: Box::new(self.cond.clone()),
        })
    }
}

impl Parse for Toggle {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: bracketed!(content in input),
            cond: content.parse()?,
        })
    }
}

impl ToTokens for Toggle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket_token.surround(tokens, |tokens| {
            self.cond.to_tokens(tokens);
        });
    }
}

#[derive(Debug, Clone)]
struct Keyword<N> {
    at_token: At,
    kind: KeywordKind<N>,
}

impl<N: Node> Parse for Keyword<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            at_token: input.parse()?,
            kind: input.parse()?,
        })
    }
}

impl<N: Node> ToTokens for Keyword<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.at_token.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl<N: Node> Generate for Keyword<N> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.kind);
    }
}

#[derive(Debug, Clone)]
enum KeywordKind<N> {
    Let(Local),
    If(IfNode<N>),
    For(ForNode<N>),
    While(WhileNode<N>),
    Match(MatchNode<N>),
}

impl<N: Node> Parse for KeywordKind<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![let]) {
            let Stmt::Local(local) = input.parse()? else {
                unreachable!()
            };

            Ok(Self::Let(local))
        } else if lookahead.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if lookahead.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if lookahead.peek(Token![while]) {
            Ok(Self::While(input.parse()?))
        } else if lookahead.peek(Token![match]) {
            Ok(Self::Match(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl<N: Node> ToTokens for KeywordKind<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Let(let_) => let_.to_tokens(tokens),
            Self::If(if_) => if_.to_tokens(tokens),
            Self::For(for_) => for_.to_tokens(tokens),
            Self::While(while_) => while_.to_tokens(tokens),
            Self::Match(match_) => match_.to_tokens(tokens),
        }
    }
}

impl<N: Node> Generate for KeywordKind<N> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Let(let_) => g.push_dynamic(Stmt::Local(let_.clone()), Some(self.span())),
            Self::If(if_) => g.push(if_),
            Self::For(for_) => g.push(for_),
            Self::While(while_) => g.push(while_),
            Self::Match(match_) => g.push(match_),
        }
    }
}

#[derive(Debug, Clone)]
struct IfNode<N> {
    if_token: If,
    cond: Expr,
    then_branch: Block<N>,
    else_branch: Option<(At, Else, Box<IfOrBlock<N>>)>,
}

impl<N: Node> Parse for IfNode<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            then_branch: input.parse()?,
            else_branch: {
                if input.peek(Token![@]) && input.peek2(Token![else]) {
                    Some((input.parse()?, input.parse()?, input.parse()?))
                } else {
                    None
                }
            },
        })
    }
}

impl<N: Node> ToTokens for IfNode<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.if_token.to_tokens(tokens);
        self.cond.to_tokens(tokens);
        self.then_branch.to_tokens(tokens);
        if let Some((at_token, else_token, else_branch)) = &self.else_branch {
            at_token.to_tokens(tokens);
            else_token.to_tokens(tokens);
            else_branch.to_tokens(tokens);
        }
    }
}

impl<N: Node> Generate for IfNode<N> {
    fn generate(&self, g: &mut Generator) {
        fn to_expr<N: Node>(if_: &IfNode<N>, g: &mut Generator) -> ExprIf {
            ExprIf {
                attrs: Vec::new(),
                if_token: if_.if_token,
                cond: Box::new(if_.cond.clone()),
                then_branch: g.block(&if_.then_branch),
                else_branch: if_
                    .else_branch
                    .as_ref()
                    .map(|(_, else_token, if_or_block)| {
                        (
                            *else_token,
                            Box::new(match &**if_or_block {
                                IfOrBlock::If(if_) => Expr::If(to_expr(if_, g)),
                                IfOrBlock::Block(block) => Expr::Block(ExprBlock {
                                    attrs: Vec::new(),
                                    label: None,
                                    block: g.block(block),
                                }),
                            }),
                        )
                    }),
            }
        }

        let expr = to_expr(self, g);

        g.push_expr(expr);
    }
}

#[derive(Debug, Clone)]
enum IfOrBlock<N> {
    If(IfNode<N>),
    Block(Block<N>),
}

impl<N: Node> Parse for IfOrBlock<N> {
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

impl<N: Node> ToTokens for IfOrBlock<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::If(if_) => if_.to_tokens(tokens),
            Self::Block(block) => block.to_tokens(tokens),
        }
    }
}

impl<N: Node> Generate for IfOrBlock<N> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::If(if_) => g.push(if_),
            Self::Block(block) => g.push(block),
        }
    }
}

#[derive(Debug, Clone)]
struct ForNode<N> {
    for_token: For,
    pat: Pat,
    in_token: In,
    expr: Expr,
    body: Block<N>,
}

impl<N: Node> Parse for ForNode<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            in_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            body: input.parse()?,
        })
    }
}

impl<N: Node> ToTokens for ForNode<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.for_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.in_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl<N: Node> Generate for ForNode<N> {
    fn generate(&self, g: &mut Generator) {
        let body = g.block(&self.body);
        g.push_expr(ExprForLoop {
            attrs: Vec::new(),
            label: None,
            for_token: self.for_token,
            pat: Box::new(self.pat.clone()),
            in_token: self.in_token,
            expr: Box::new(self.expr.clone()),
            body,
        });
    }
}

#[derive(Debug, Clone)]
struct WhileNode<N> {
    while_token: While,
    cond: Expr,
    body: Block<N>,
}

impl<N: Node> Parse for WhileNode<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            while_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            body: input.parse()?,
        })
    }
}

impl<N: Node> ToTokens for WhileNode<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.while_token.to_tokens(tokens);
        self.cond.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl<N: Node> Generate for WhileNode<N> {
    fn generate(&self, g: &mut Generator) {
        let body = g.block(&self.body);
        g.push_expr(ExprWhile {
            attrs: Vec::new(),
            label: None,
            while_token: self.while_token,
            cond: Box::new(self.cond.clone()),
            body,
        });
    }
}

#[derive(Debug, Clone)]
struct MatchNode<N> {
    match_token: Match,
    expr: Expr,
    brace_token: Brace,
    arms: Vec<MatchNodeArm<N>>,
}

impl<N: Node> Parse for MatchNode<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let match_token = input.parse()?;
        let expr = input.call(Expr::parse_without_eager_brace)?;

        let content;
        let brace_token = braced!(content in input);

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(content.parse()?);
        }

        Ok(Self {
            match_token,
            expr,
            brace_token,
            arms,
        })
    }
}

impl<N: Node> ToTokens for MatchNode<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.match_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            for arm in &self.arms {
                arm.to_tokens(tokens);
            }
        });
    }
}

impl<N: Node> Generate for MatchNode<N> {
    fn generate(&self, g: &mut Generator) {
        let arms = self
            .arms
            .iter()
            .map(|arm| Arm {
                attrs: Vec::new(),
                pat: arm.pat.clone(),
                guard: arm
                    .guard
                    .as_ref()
                    .map(|(if_token, guard)| (*if_token, Box::new(guard.clone()))),
                fat_arrow_token: arm.fat_arrow_token,
                body: Box::new(Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block: g.block(&arm.body),
                })),
                comma: arm.comma_token,
            })
            .collect();

        g.push_expr(ExprMatch {
            attrs: Vec::new(),
            match_token: self.match_token,
            expr: Box::new(self.expr.clone()),
            brace_token: self.brace_token,
            arms,
        });
    }
}

#[derive(Debug, Clone)]
struct MatchNodeArm<N> {
    pat: Pat,
    guard: Option<(If, Expr)>,
    fat_arrow_token: FatArrow,
    body: N,
    comma_token: Option<Comma>,
}

impl<N: Node> Parse for MatchNodeArm<N> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            guard: {
                if input.peek(Token![if]) {
                    Some((input.parse()?, input.parse()?))
                } else {
                    None
                }
            },
            fat_arrow_token: input.parse()?,
            body: input.parse()?,
            comma_token: if input.peek(Token![,]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl<N: Node> ToTokens for MatchNodeArm<N> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        if let Some((if_token, guard)) = &self.guard {
            if_token.to_tokens(tokens);
            guard.to_tokens(tokens);
        }
        self.fat_arrow_token.to_tokens(tokens);
        self.body.to_tokens(tokens);
        if let Some(comma_token) = &self.comma_token {
            comma_token.to_tokens(tokens);
        }
    }
}
