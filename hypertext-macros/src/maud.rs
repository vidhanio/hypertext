#![allow(clippy::struct_field_names)]

use std::ops::ControlFlow;

use proc_macro2::TokenStream;
use syn::{
    Arm, Expr, ExprBlock, ExprForLoop, ExprIf, ExprMatch, ExprParen, ExprWhile, Ident, LitBool,
    LitInt, LitStr, Local, Pat, Stmt, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::{Pair, Punctuated},
    spanned::Spanned,
    token::{Brace, Bracket, Paren},
};
use syn_derive::{Parse, ToTokens};

use crate::generate::{Generate, Generator};

pub fn parse(tokens: TokenStream) -> syn::Result<Markup> {
    syn::parse2(tokens)
}

fn parse_option<T: Parse>(
    input: ParseStream,
    peek: fn(ParseStream) -> bool,
) -> syn::Result<Option<T>> {
    if peek(input) {
        Ok(Some(input.parse()?))
    } else {
        Ok(None)
    }
}

fn parse_until_empty<T: Parse>(input: ParseStream) -> syn::Result<Vec<T>> {
    let mut vec = Vec::new();
    while !input.is_empty() {
        vec.push(input.parse()?);
    }
    Ok(vec)
}

#[derive(Parse)]
pub struct Markup {
    #[parse(|input| {
        parse_option(input, |input| input.peek(Token![!]) && input.peek2(DOCTYPE))
    })]
    doctype: Option<Doctype>,

    #[parse(|input| parse_until_empty(input))]
    nodes: Vec<ElementNode>,
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

#[derive(Parse)]
struct Doctype {
    #[allow(dead_code)]
    bang_token: Token![!],
    name: DOCTYPE,
}

impl Generate for Doctype {
    fn generate(&self, g: &mut Generator) {
        g.push_spanned_str("<!DOCTYPE html>", self.name.span());
    }
}

trait Node: Parse + Generate {
    fn is_let(&self) -> bool;
}

enum ElementNode {
    Block(Block<Self>),
    Element(Element),
    Splice(Splice),
    Literal(Lit),
    Keyword(Keyword<Self>),
}

// not derivable because `syn_derive` doesn't support multiple lookaheads, like
// used in `Literal`
impl Parse for ElementNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            input.parse().map(Self::Block)
        } else if lookahead.peek(Ident::peek_any) {
            input.parse().map(Self::Element)
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

#[derive(Parse)]
struct Block<N: Node> {
    #[syn(braced)]
    #[allow(dead_code)]
    brace_token: Brace,

    #[syn(in = brace_token)]
    #[parse(parse_until_empty)]
    nodes: Vec<N>,
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

#[derive(Parse)]
struct Splice {
    #[syn(parenthesized)]
    #[allow(dead_code)]
    paren_token: Paren,

    #[syn(in = paren_token)]
    expr: Expr,
}

impl Generate for Splice {
    fn generate(&self, g: &mut Generator) {
        g.push_rendered_expr(&self.expr);
    }
}

#[derive(Parse)]
struct Element {
    name: Name,

    #[parse(|input| parse_option(input, |input| input.peek(Token![#])))]
    id: Option<IdAttribute>,

    #[parse(|input| {
        let mut classes = Vec::new();

        while input.peek(Token![.]) {
            classes.push(input.parse()?);
        }

        Ok(classes)
    })]
    classes: Vec<Class>,

    #[parse(|input| {
        let mut attrs = Vec::new();

        while !(input.peek(Token![;]) || input.peek(Brace)) {
            attrs.push(input.parse()?);
        }

        Ok(attrs)
    })]
    attrs: Vec<Attribute>,

    body: ElementBody,
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

        if let Some(class) = self.classes.first() {
            g.record_attribute(&self.name.ident(), &class.attr_name_ident());

            g.push_str(" ");
            g.push_escaped_lit(class.attr_name_lit());
            g.push_str("=\"");

            for (i, class) in self.classes.iter().enumerate() {
                if let Some(toggle) = &class.toggle {
                    g.push_conditional(&toggle.cond_expr(), |g| {
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

        for attr in &self.attrs {
            g.push(attr);

            let mut name_pairs = attr.name.name.pairs();
            let is_data = name_pairs.next().is_some_and(|pair| {
                matches!(pair, Pair::Punctuated(NameFragment::Ident(ident), NamePunct::Hyphen(_)) if ident == "data")
            });

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

#[derive(Parse)]
enum ElementBody {
    #[parse(peek = Token![;])]
    Void(#[allow(dead_code)] Token![;]),

    #[parse(peek = Brace)]
    Block(Block<ElementNode>),
}

#[derive(Parse)]
struct IdAttribute {
    pound_token: Token![#],
    value: IdOrClassNode,
}

impl IdAttribute {
    fn attr_name_ident(&self) -> Ident {
        Ident::new("id", self.pound_token.span())
    }

    fn attr_name_lit(&self) -> LitStr {
        LitStr::new("id", self.pound_token.span())
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

#[derive(Parse)]
struct Class {
    dot_token: Token![.],
    value: IdOrClassNode,
    #[parse(|input| parse_option(input, |tokens| tokens.peek(Bracket)))]
    toggle: Option<Toggle>,
}

impl Class {
    fn attr_name_ident(&self) -> Ident {
        Ident::new("class", self.dot_token.span())
    }

    fn attr_name_lit(&self) -> LitStr {
        LitStr::new("class", self.dot_token.span())
    }
}

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

// not derivable because `syn_derive` doesn't support multiple lookaheads, like
// used in `Name`
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

#[derive(Parse)]
struct Attribute {
    name: Name,
    kind: AttributeKind,
}

impl Generate for Attribute {
    fn generate(&self, g: &mut Generator) {
        match &self.kind {
            AttributeKind::Normal {
                value,
                toggle: Some(toggle),
                ..
            } => g.push_conditional(&toggle.cond_expr(), |g| {
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
                g.push_conditional(&toggle.cond_expr(), |g| {
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

enum AttributeKind {
    Normal {
        #[allow(dead_code)]
        eq_token: Token![=],
        value: AttributeValueNode,
        toggle: Option<Toggle>,
    },
    Optional {
        #[allow(dead_code)]
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
                    toggle: parse_option(input, |input| input.peek(Bracket))?,
                })
            }
        } else {
            Ok(Self::Empty(parse_option(input, |input| {
                input.peek(Bracket)
            })?))
        }
    }
}

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

// not derivable because `syn_derive` doesn't support multiple lookaheads, like
// used in `Literal`
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

#[derive(ToTokens)]
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

#[derive(ToTokens)]
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

#[derive(Parse, ToTokens)]
enum NamePunct {
    #[parse(peek = Token![:])]
    Colon(Token![:]),

    #[parse(peek = Token![-])]
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

#[derive(Parse)]
enum Lit {
    #[parse(peek = LitStr)]
    Str(LitStr),

    #[parse(peek = LitInt)]
    Int(LitInt),

    #[parse(peek = LitBool)]
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

impl Generate for Lit {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(self.lit_str());
    }
}

#[derive(Parse)]
struct Toggle {
    #[syn(bracketed)]
    #[allow(dead_code)]
    bracket_token: Bracket,

    #[syn(in = bracket_token)]
    cond: Expr,
}

impl Toggle {
    fn cond_expr(&self) -> Expr {
        Expr::Paren(ExprParen {
            attrs: Vec::new(),
            paren_token: Paren::default(),
            expr: Box::new(self.cond.clone()),
        })
    }
}

#[derive(Parse)]
struct Keyword<N: Node> {
    #[allow(dead_code)]
    at_token: Token![@],
    kind: KeywordKind<N>,
}

impl<N: Node> Generate for Keyword<N> {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.kind);
    }
}

enum KeywordKind<N: Node> {
    Let(Local),
    If(IfNode<N>),
    For(ForNode<N>),
    While(WhileNode<N>),
    Match(MatchNode<N>),
}

// not derivable because `syn_derive` doesn't support custom parsing in enum
// variants
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

impl<N: Node> Generate for KeywordKind<N> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Let(let_) => g.push_dynamic(Stmt::Local(let_.clone()), Some(let_.span())),
            Self::If(if_) => g.push(if_),
            Self::For(for_) => g.push(for_),
            Self::While(while_) => g.push(while_),
            Self::Match(match_) => g.push(match_),
        }
    }
}

#[derive(Parse)]
struct IfNode<N: Node> {
    if_token: Token![if],

    #[parse(Expr::parse_without_eager_brace)]
    cond: Expr,

    then_branch: Block<N>,

    #[parse(|input| {
        if input.peek(Token![@]) && input.peek2(Token![else]) {
            Ok(Some((input.parse()?, input.parse()?, input.parse()?)))
        } else {
            Ok(None)
        }
    })]
    else_branch: Option<(Token![@], Token![else], Box<IfOrBlock<N>>)>,
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

#[derive(Parse)]
enum IfOrBlock<N: Node> {
    #[parse(peek = Token![if])]
    If(IfNode<N>),

    #[parse(peek = Brace)]
    Block(Block<N>),
}

impl<N: Node> Generate for IfOrBlock<N> {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::If(if_) => g.push(if_),
            Self::Block(block) => g.push(block),
        }
    }
}

#[derive(Parse)]
struct ForNode<N: Node> {
    for_token: Token![for],

    #[parse(Pat::parse_multi_with_leading_vert)]
    pat: Pat,

    in_token: Token![in],

    #[parse(Expr::parse_without_eager_brace)]
    expr: Expr,

    body: Block<N>,
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

#[derive(Parse)]
struct WhileNode<N: Node> {
    while_token: Token![while],

    #[parse(Expr::parse_without_eager_brace)]
    cond: Expr,

    body: Block<N>,
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

#[derive(Parse)]
struct MatchNode<N: Node> {
    match_token: Token![match],

    #[parse(Expr::parse_without_eager_brace)]
    expr: Expr,

    #[syn(braced)]
    brace_token: Brace,

    #[syn(in = brace_token)]
    #[parse(parse_until_empty)]
    arms: Vec<MatchNodeArm<N>>,
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

#[derive(Parse)]
struct MatchNodeArm<N: Node> {
    #[parse(Pat::parse_multi_with_leading_vert)]
    pat: Pat,

    #[parse(|input| {
        if input.peek(Token![if]) {
            Ok(Some((input.parse()?, input.parse()?)))
        } else {
            Ok(None)
        }
    })]
    guard: Option<(Token![if], Expr)>,

    fat_arrow_token: Token![=>],

    body: N,

    #[parse(|input| parse_option(input, |input| input.peek(Token![,])))]
    comma_token: Option<Token![,]>,
}
