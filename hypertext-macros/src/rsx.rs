use std::ops::ControlFlow;

use proc_macro2::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::ToTokens;
use rstml::{
    Parser, ParserConfig,
    node::{
        AttributeValueExpr, CustomNode, KVAttributeValue, KeyedAttribute, KeyedAttributeValue,
        NodeAttribute, NodeBlock, NodeComment, NodeDoctype, NodeElement, NodeFragment, NodeName,
        NodeNameFragment, NodeText, RawText,
    },
    recoverable::{ParseRecoverable, RecoverableContext},
};
use syn::{
    Arm, Expr, ExprBlock, ExprForLoop, ExprIf, ExprLit, ExprMatch, ExprPath, ExprWhile, Ident, Lit,
    LitStr, Local, Pat, Stmt, Token, braced,
    parse::ParseStream,
    punctuated::Pair,
    spanned::Spanned,
    token::{At, Brace, Comma, Else, FatArrow, For, If, In, Match, While},
};

use crate::generate::{Generate, Generator};

type Node = rstml::node::Node<NodeKeyword>;

pub fn parse(tokens: TokenStream) -> (Vec<Node>, Vec<Diagnostic>) {
    let void_elements = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
        "track", "wbr",
    ]
    .into_iter()
    .collect();

    let config = ParserConfig::new()
        .always_self_closed_elements(void_elements)
        .custom_node::<NodeKeyword>();

    let parser = Parser::new(config);
    let (parsed_nodes, mut diagnostics) = parser.parse_recoverable(tokens).split_vec();
    for el in parsed_nodes
        .clone()
        .into_iter()
        .flat_map(Node::flatten)
        .filter_map(|node| {
            if let Node::Element(el) = node {
                Some(el)
            } else {
                None
            }
        })
    {
        if let NodeName::Block(block) = el.open_tag.name {
            diagnostics.push(block.span().error("block names are unsupported"));
        }

        for attr in el.open_tag.attributes {
            match attr {
                NodeAttribute::Block(block) => {
                    diagnostics.push(block.span().error("block attributes are unsupported"));
                }
                NodeAttribute::Attribute(attr) => {
                    match attr.key {
                        NodeName::Block(block) => {
                            diagnostics
                                .push(block.span().error("block attribute keys are unsupported"));
                        }
                        NodeName::Path(path) => {
                            if let Some(qself) = path.qself {
                                diagnostics
                                    .push(qself.span().error("qualified self is unsupported"));
                            }

                            if let Some(leading_colon) = path.path.leading_colon {
                                diagnostics.push(
                                    leading_colon.span().error("leading colons are unsupported"),
                                );
                            }
                        }
                        NodeName::Punctuated(punctuated) => {
                            if !punctuated.pairs().all(|pair| {
                                pair.punct().is_none_or(|punct| {
                                    punct.as_char() == '-' || punct.as_char() == ':'
                                })
                            }) {
                                diagnostics.push(
                                    punctuated
                                        .span()
                                        .error("only dashes and colons are supported"),
                                );
                            }
                        }
                    }

                    if let KeyedAttributeValue::Binding(b) = attr.possible_value {
                        diagnostics.push(b.span().error("function bindings are unsupported"));
                    }
                }
            }
        }
    }

    (parsed_nodes, diagnostics)
}

impl Generate for Vec<Node> {
    fn generate(&self, g: &mut Generator) {
        g.push_all(self);
    }
}

impl Generate for Node {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Comment(comment) => g.push(comment),
            Self::Doctype(doctype) => g.push(doctype),
            Self::Fragment(fragment) => g.push(fragment),
            Self::Element(element) => g.push(element),
            Self::Block(block) => g.push(block),
            Self::Text(text) => g.push(text),
            Self::RawText(raw_text) => g.push(raw_text),
            Self::Custom(keyword) => g.push(keyword),
        }
    }
}

impl Generate for NodeComment {
    fn generate(&self, g: &mut Generator) {
        g.push_str("<!--");
        g.push_escaped_lit(self.value.clone());
        g.push_str("-->");
    }
}

impl Generate for NodeDoctype {
    fn generate(&self, g: &mut Generator) {
        g.push_str("<!");
        g.push_spanned_str("DOCTYPE", self.token_doctype.span());
        g.push_str(" ");
        g.push(&self.value);
        g.push_str(">");
    }
}

impl Generate for NodeFragment<NodeKeyword> {
    fn generate(&self, g: &mut Generator) {
        g.push_all(&self.children);
    }
}

impl Generate for NodeElement<NodeKeyword> {
    fn generate(&self, g: &mut Generator) {
        g.record_element(&node_name_ident(&self.open_tag.name));

        g.push_str("<");
        g.push_escaped_lit(node_name_lit(&self.open_tag.name));
        for attr in &self.open_tag.attributes {
            let NodeAttribute::Attribute(attr) = attr else {
                continue;
            };

            g.push(attr);

            let is_data = if let KeyedAttribute {
                key: NodeName::Punctuated(punct),
                ..
            } = attr
            {
                let mut name_pairs = punct.pairs();

                name_pairs.next().is_some_and(|pair| {
                    if let Pair::Punctuated(NodeNameFragment::Ident(ident), punct) = pair {
                        ident == "data" && punct.as_char() == '-'
                    } else {
                        false
                    }
                }) && name_pairs.next().is_some()
            } else {
                false
            };

            if !is_data {
                let (attr_ident, is_namespace) = node_name_ident_or_namespace(&attr.key);

                if is_namespace {
                    g.record_namespace(&node_name_ident(&self.open_tag.name), &attr_ident);
                } else {
                    g.record_attribute(&node_name_ident(&self.open_tag.name), &attr_ident);
                }
            }
        }
        g.push_str(">");

        if let Some(tag) = &self.close_tag {
            g.record_element(&node_name_ident(&tag.name));
            g.push_all(&self.children);

            g.push_str("</");
            g.push_escaped_lit(node_name_lit(&tag.name));
            g.push_str(">");
        } else {
            g.record_void_element(&node_name_ident(&self.open_tag.name));
        }
    }
}

impl Generate for KeyedAttribute {
    fn generate(&self, g: &mut Generator) {
        g.push_str(" ");

        g.push_escaped_lit(node_name_lit(&self.key));

        if let KeyedAttributeValue::Value(AttributeValueExpr {
            value: KVAttributeValue::Expr(value),
            ..
        }) = &self.possible_value
        {
            g.push_str("=\"");
            match value {
                Expr::Lit(ExprLit { lit, .. }) => match lit {
                    Lit::Str(lit_str) => {
                        g.push_escaped_lit(lit_str.clone());
                    }
                    Lit::Int(lit_int) => {
                        g.push_escaped_lit(LitStr::new(&lit_int.to_string(), lit_int.span()));
                    }
                    Lit::Bool(lit_bool) => {
                        g.push_escaped_lit(LitStr::new(
                            &lit_bool.value.to_string(),
                            lit_bool.span(),
                        ));
                    }
                    _ => {
                        g.push_rendered_expr(value);
                    }
                },
                _ => {
                    g.push_rendered_expr(value);
                }
            }
            g.push_str("\"");
        }
    }
}

impl Generate for NodeBlock {
    fn generate(&self, g: &mut Generator) {
        if let Self::ValidBlock(block) = self {
            g.push_rendered_expr(&Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: block.clone(),
            }));
        }
    }
}

impl Generate for NodeText {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(self.value.clone());
    }
}

impl Generate for RawText {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(LitStr::new(&self.to_string_best(), self.span()));
    }
}

impl Generate for RawText<NodeKeyword> {
    fn generate(&self, g: &mut Generator) {
        g.push_escaped_lit(LitStr::new(&self.to_string_best(), self.span()));
    }
}

#[derive(Debug, Clone)]
pub struct NodeKeyword {
    at_token: At,
    kind: KeywordKind,
}

impl CustomNode for NodeKeyword {
    fn peek_element(input: syn::parse::ParseStream) -> bool {
        input.peek(Token![@])
            && (input.peek2(Token![let])
                || input.peek2(Token![while])
                || input.peek2(Token![loop])
                || input.peek2(Token![if])
                || input.peek2(Token![for])
                || input.peek2(Token![match]))
    }
}

impl ParseRecoverable for NodeKeyword {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        Some(Self {
            at_token: parser.parse_simple(input)?,
            kind: parser.parse_recoverable(input)?,
        })
    }
}

impl ToTokens for NodeKeyword {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.at_token.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl Generate for NodeKeyword {
    fn generate(&self, g: &mut Generator) {
        g.push(&self.kind);
    }
}

#[derive(Debug, Clone)]
enum KeywordKind {
    Let(Local),
    If(NodeIf),
    For(NodeFor),
    While(NodeWhile),
    Match(NodeMatch),
}

impl ParseRecoverable for KeywordKind {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        if input.peek(Token![let]) {
            let Stmt::Local(local) = parser.parse_simple(input)? else {
                return None;
            };

            Some(Self::Let(local))
        } else if input.peek(Token![if]) {
            Some(Self::If(parser.parse_recoverable(input)?))
        } else if input.peek(Token![for]) {
            Some(Self::For(parser.parse_recoverable(input)?))
        } else if input.peek(Token![while]) {
            Some(Self::While(parser.parse_recoverable(input)?))
        } else if input.peek(Token![match]) {
            Some(Self::Match(parser.parse_recoverable(input)?))
        } else {
            None
        }
    }
}

impl ToTokens for KeywordKind {
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

impl Generate for KeywordKind {
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
struct KeywordBlock {
    brace_token: Brace,
    nodes: Vec<Node>,
}

impl ParseRecoverable for KeywordBlock {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        parser.parse_mixed_fn(input, |parser, input| {
            let content;
            let brace_token = braced!(content in input);
            let mut nodes = vec![];
            while !content.is_empty() {
                let Some(node) = parser.parse_recoverable(&content) else {
                    return Ok(None);
                };
                nodes.push(node);
            }
            Ok(Some(Self { brace_token, nodes }))
        })?
    }
}

impl ToTokens for KeywordBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.brace_token.surround(tokens, |tokens| {
            for node in &self.nodes {
                node.to_tokens(tokens);
            }
        });
    }
}

impl Generate for KeywordBlock {
    fn generate(&self, g: &mut Generator) {
        if self.nodes.iter().any(|node| {
            matches!(
                node,
                Node::Custom(NodeKeyword {
                    kind: KeywordKind::Let(_),
                    ..
                })
            )
        }) {
            g.in_block(|g| g.push_all(&self.nodes));
        } else {
            g.push_all(&self.nodes);
        }
    }
}

#[derive(Debug, Clone)]
struct NodeIf {
    if_token: If,
    cond: Expr,
    then_branch: KeywordBlock,
    else_branch: Option<(At, Else, Box<NodeIfOrBlock>)>,
}

impl ParseRecoverable for NodeIf {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        Some(Self {
            if_token: parser.parse_simple(input)?,
            cond: parser.parse_mixed_fn(input, |_, input| {
                input.call(Expr::parse_without_eager_brace)
            })?,
            then_branch: parser.parse_recoverable(input)?,
            else_branch: if input.peek(Token![@]) && input.peek2(Token![else]) {
                Some((
                    parser.parse_simple(input)?,
                    parser.parse_simple(input)?,
                    Box::new(parser.parse_recoverable(input)?),
                ))
            } else {
                None
            },
        })
    }
}

impl ToTokens for NodeIf {
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

impl Generate for NodeIf {
    fn generate(&self, g: &mut Generator) {
        fn to_expr(if_: &NodeIf, g: &mut Generator) -> ExprIf {
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
                                NodeIfOrBlock::If(if_) => Expr::If(to_expr(if_, g)),
                                NodeIfOrBlock::Block(block) => Expr::Block(ExprBlock {
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
enum NodeIfOrBlock {
    If(NodeIf),
    Block(KeywordBlock),
}

impl ParseRecoverable for NodeIfOrBlock {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        if input.peek(Token![if]) {
            parser.parse_recoverable(input).map(Self::If)
        } else if input.peek(Brace) {
            parser.parse_recoverable(input).map(Self::Block)
        } else {
            None
        }
    }
}

impl ToTokens for NodeIfOrBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::If(if_) => if_.to_tokens(tokens),
            Self::Block(block) => block.to_tokens(tokens),
        }
    }
}

impl Generate for NodeIfOrBlock {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::If(if_) => g.push(if_),
            Self::Block(block) => g.push(block),
        }
    }
}

#[derive(Debug, Clone)]
struct NodeFor {
    for_token: For,
    pat: Pat,
    in_token: In,
    expr: Expr,
    body: KeywordBlock,
}

impl ParseRecoverable for NodeFor {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        Some(Self {
            for_token: parser.parse_simple(input)?,
            pat: parser.parse_mixed_fn(input, |_, input| {
                input.call(Pat::parse_multi_with_leading_vert)
            })?,
            in_token: parser.parse_simple(input)?,
            expr: parser.parse_mixed_fn(input, |_, input| {
                input.call(Expr::parse_without_eager_brace)
            })?,
            body: parser.parse_recoverable(input)?,
        })
    }
}

impl ToTokens for NodeFor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.for_token.to_tokens(tokens);
        self.pat.to_tokens(tokens);
        self.in_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}
impl Generate for NodeFor {
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
struct NodeWhile {
    while_token: While,
    cond: Expr,
    body: KeywordBlock,
}

impl ParseRecoverable for NodeWhile {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        Some(Self {
            while_token: parser.parse_simple(input)?,
            cond: parser.parse_mixed_fn(input, |_, input| {
                input.call(Expr::parse_without_eager_brace)
            })?,
            body: parser.parse_recoverable(input)?,
        })
    }
}

impl ToTokens for NodeWhile {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.while_token.to_tokens(tokens);
        self.cond.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl Generate for NodeWhile {
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
struct NodeMatch {
    match_token: Match,
    expr: Expr,
    brace_token: Brace,
    arms: Vec<NodeMatchArm>,
}

impl ParseRecoverable for NodeMatch {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        parser.parse_mixed_fn(input, |parser, input| {
            let Some(match_token) = parser.parse_simple(input) else {
                return Ok(None);
            };
            let expr = Expr::parse_without_eager_brace(input)?;
            let content;
            let brace_token = braced!(content in input);
            let mut arms = vec![];
            while !content.is_empty() {
                let Some(arm) = parser.parse_recoverable(&content) else {
                    return Ok(None);
                };
                arms.push(arm);
            }

            Ok(Some(Self {
                match_token,
                expr,
                brace_token,
                arms,
            }))
        })?
    }
}

impl ToTokens for NodeMatch {
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

impl Generate for NodeMatch {
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
struct NodeMatchArm {
    pat: Pat,
    guard: Option<(If, Expr)>,
    fat_arrow_token: FatArrow,
    body: NodeMatchArmBody,
    comma_token: Option<Comma>,
}

impl ParseRecoverable for NodeMatchArm {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        Some(Self {
            pat: parser.parse_mixed_fn(input, |_, input| {
                input.call(Pat::parse_multi_with_leading_vert)
            })?,
            guard: if input.peek(Token![if]) {
                Some((parser.parse_simple(input)?, parser.parse_simple(input)?))
            } else {
                None
            },
            fat_arrow_token: parser.parse_simple(input)?,
            body: parser.parse_recoverable(input)?,
            comma_token: if input.peek(Token![,]) {
                Some(parser.parse_simple(input)?)
            } else {
                None
            },
        })
    }
}

impl ToTokens for NodeMatchArm {
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

#[derive(Debug, Clone)]
enum NodeMatchArmBody {
    Block(KeywordBlock),
    Node(Node),
}

impl ParseRecoverable for NodeMatchArmBody {
    fn parse_recoverable(parser: &mut RecoverableContext, input: ParseStream) -> Option<Self> {
        if input.peek(Brace) {
            parser.parse_recoverable(input).map(Self::Block)
        } else {
            parser.parse_recoverable(input).map(Self::Node)
        }
    }
}

impl ToTokens for NodeMatchArmBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Block(block) => block.to_tokens(tokens),
            Self::Node(node) => node.to_tokens(tokens),
        }
    }
}

impl Generate for NodeMatchArmBody {
    fn generate(&self, g: &mut Generator) {
        match self {
            Self::Block(block) => g.push(block),
            Self::Node(node) => g.push(node),
        }
    }
}

fn node_name_ident(node_name: &NodeName) -> Ident {
    match node_name {
        NodeName::Path(ExprPath { path, .. }) => path.segments.last().map_or_else(
            || Ident::new("_", path.span()),
            |segment| {
                syn::parse2::<Ident>(segment.ident.to_token_stream()).map_or_else(
                    |_| Ident::new_raw(&segment.ident.to_string(), segment.ident.span()),
                    |mut ident| {
                        ident.set_span(segment.ident.span());
                        ident
                    },
                )
            },
        ),
        NodeName::Punctuated(punctuated) => {
            let string = punctuated.pairs().map(Pair::into_tuple).fold(
                String::new(),
                |mut acc, (ident, punct)| {
                    acc.push_str(&ident.to_string());
                    if punct.is_some() {
                        acc.push('_');
                    }
                    acc
                },
            );

            syn::parse_str::<Ident>(&string).map_or_else(
                |_| Ident::new_raw(&string, punctuated.span()),
                |mut ident| {
                    ident.set_span(punctuated.span());
                    ident
                },
            )
        }
        NodeName::Block(_) => Ident::new("_", node_name.span()),
    }
}

fn node_name_ident_or_namespace(node_name: &NodeName) -> (Ident, bool) {
    match node_name {
        NodeName::Path(ExprPath { path, .. }) => match path.segments.len() {
            0 => (Ident::new("_", node_name.span()), false),
            l => {
                let segment = path.segments.first().unwrap();
                let ident = syn::parse2::<Ident>(segment.ident.to_token_stream()).map_or_else(
                    |_| Ident::new_raw(&segment.ident.to_string(), node_name.span()),
                    |mut ident| {
                        ident.set_span(node_name.span());
                        ident
                    },
                );
                let is_namespace = l > 1;
                (ident, is_namespace)
            }
        },
        NodeName::Punctuated(punctuated) => {
            let string = punctuated.pairs().map(Pair::into_tuple).try_fold(
                String::new(),
                |mut acc, (fragment, punct)| {
                    acc.push_str(&fragment.to_string());

                    if let Some(punct) = punct {
                        if punct.as_char() == ':' {
                            return ControlFlow::Break(acc);
                        } else if punct.as_char() == '-' {
                            acc.push('_');
                        }
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
                    |_| Ident::new_raw(&string, node_name.span()),
                    |mut ident| {
                        ident.set_span(node_name.span());
                        ident
                    },
                ),
                is_namespace,
            )
        }
        NodeName::Block(_) => (Ident::new("_", node_name.span()), false),
    }
}

fn node_name_lit(node_name: &NodeName) -> LitStr {
    match node_name {
        NodeName::Path(ExprPath { path, .. }) => {
            let string =
                path.segments
                    .iter()
                    .enumerate()
                    .fold(String::new(), |mut acc, (i, segment)| {
                        if i > 0 {
                            acc.push_str("::");
                        }
                        acc.push_str(&segment.ident.to_string());
                        acc
                    });

            LitStr::new(&string, node_name.span())
        }
        NodeName::Punctuated(punctuated) => {
            let string = punctuated.pairs().map(Pair::into_tuple).fold(
                String::new(),
                |mut acc, (ident, punct)| {
                    acc.push_str(&ident.to_string());
                    if let Some(punct) = punct {
                        acc.push(punct.as_char());
                    }
                    acc
                },
            );

            LitStr::new(&string, node_name.span())
        }
        NodeName::Block(_) => LitStr::new("", node_name.span()),
    }
}
