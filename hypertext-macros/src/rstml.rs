use std::collections::HashSet;

use proc_macro2::TokenStream;
use proc_macro2_diagnostics::{Diagnostic, SpanDiagnosticExt};
use quote::ToTokens;
use rstml::{
    node::{
        AttributeValueExpr, KVAttributeValue, KeyedAttribute, KeyedAttributeValue, Node,
        NodeAttribute, NodeBlock, NodeComment, NodeDoctype, NodeElement, NodeFragment, NodeName,
        NodeNameFragment, NodeText, RawText,
    },
    Infallible, Parser, ParserConfig,
};
use syn::{
    punctuated::Pair, spanned::Spanned, Expr, ExprBlock, ExprLit, ExprPath, Ident, Lit, LitStr,
};

use crate::generate::{Generate, Generator};

pub fn parse(tokens: TokenStream) -> (Vec<Node>, Vec<Diagnostic>) {
    let void_elements = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source",
        "track", "wbr",
    ]
    .into_iter()
    .collect::<HashSet<_>>();

    let config = ParserConfig::new()
        .recover_block(true)
        .always_self_closed_elements(void_elements);

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
                NodeAttribute::Attribute(keyed) => {
                    match keyed.key {
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

                            if path.path.segments.len() != 1 {
                                diagnostics.push(
                                    path.path
                                        .span()
                                        .error("multiple path segments are unsupported"),
                                );
                            }

                            if path.path.segments.trailing_punct() {
                                diagnostics.push(
                                    path.path
                                        .span()
                                        .error("trailing punctuation is unsupported"),
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

                    if let KeyedAttributeValue::Binding(b) = keyed.possible_value {
                        diagnostics.push(b.span().error("function bindings are unsupported"));
                    }
                }
            }
        }
    }

    (parsed_nodes, diagnostics)
}

impl Generate for Vec<Node> {
    fn generate(&self, gen: &mut Generator) {
        gen.push_all(self);
    }
}

impl Generate for Node {
    fn generate(&self, gen: &mut Generator) {
        match self {
            Self::Comment(comment) => gen.push(comment),
            Self::Doctype(doctype) => gen.push(doctype),
            Self::Fragment(fragment) => gen.push(fragment),
            Self::Element(element) => gen.push(element),
            Self::Block(block) => gen.push(block),
            Self::Text(text) => gen.push(text),
            Self::RawText(raw_text) => gen.push(raw_text),
            Self::Custom(_) => {}
        }
    }
}

impl Generate for NodeComment {
    fn generate(&self, gen: &mut Generator) {
        gen.push_str("<!--");
        gen.push_escaped_lit(self.value.clone());
        gen.push_str("-->");
    }
}

impl Generate for NodeDoctype {
    fn generate(&self, gen: &mut Generator) {
        gen.push_str("<!");
        gen.push_spanned_str("DOCTYPE", self.token_doctype.span());
        gen.push_str(" ");
        gen.push(&self.value);
        gen.push_str(">");
    }
}

impl Generate for NodeFragment<Infallible> {
    fn generate(&self, gen: &mut Generator) {
        gen.push_all(&self.children);
    }
}

impl Generate for NodeElement<Infallible> {
    fn generate(&self, gen: &mut Generator) {
        gen.record_element(&node_name_ident(&self.open_tag.name));

        gen.push_str("<");
        gen.push_escaped_lit(node_name_lit(&self.open_tag.name));
        for attr in &self.open_tag.attributes {
            let NodeAttribute::Attribute(attr) = attr else {
                continue;
            };

            gen.push(attr);

            if let KeyedAttribute {
                key: NodeName::Punctuated(punct),
                ..
            } = attr
            {
                let mut name_pairs = punct.pairs();
                if name_pairs.next().is_some_and(|pair| {
                    if let Pair::Punctuated(NodeNameFragment::Ident(ident), punct) = pair {
                        ident == "data" && punct.as_char() == '-'
                    } else {
                        false
                    }
                }) && name_pairs.next().is_some()
                {
                    continue;
                }
            }

            gen.record_attribute(
                &node_name_ident(&self.open_tag.name),
                &node_name_ident(&attr.key),
            );
        }
        gen.push_str(">");

        if let Some(tag) = &self.close_tag {
            gen.record_element(&node_name_ident(&tag.name));
            gen.push_all(&self.children);

            gen.push_str("</");
            gen.push_escaped_lit(node_name_lit(&tag.name));
            gen.push_str(">");
        } else {
            gen.record_void_element(&node_name_ident(&self.open_tag.name));
        }
    }
}

impl Generate for KeyedAttribute {
    fn generate(&self, gen: &mut Generator) {
        gen.push_str(" ");

        gen.push_escaped_lit(node_name_lit(&self.key));

        if let KeyedAttributeValue::Value(AttributeValueExpr {
            value: KVAttributeValue::Expr(value),
            ..
        }) = &self.possible_value
        {
            gen.push_str("=\"");
            match value {
                Expr::Lit(ExprLit { lit, .. }) => match lit {
                    Lit::Str(lit_str) => {
                        gen.push_escaped_lit(lit_str.clone());
                    }
                    Lit::Int(lit_int) => {
                        gen.push_escaped_lit(LitStr::new(&lit_int.to_string(), lit_int.span()));
                    }
                    Lit::Bool(lit_bool) => {
                        gen.push_escaped_lit(LitStr::new(
                            &lit_bool.value.to_string(),
                            lit_bool.span(),
                        ));
                    }
                    _ => {
                        gen.push_rendered_expr(value);
                    }
                },
                _ => {
                    gen.push_rendered_expr(value);
                }
            }
            gen.push_str("\"");
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

fn node_name_lit(node_name: &NodeName) -> LitStr {
    match node_name {
        NodeName::Path(ExprPath { path, .. }) => path.segments.last().map_or_else(
            || LitStr::new("", path.span()),
            |segment| LitStr::new(&segment.ident.to_string(), segment.ident.span()),
        ),
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

            LitStr::new(&string, punctuated.span())
        }
        NodeName::Block(_) => LitStr::new("", node_name.span()),
    }
}

impl Generate for NodeBlock {
    fn generate(&self, gen: &mut Generator) {
        if let Self::ValidBlock(block) = self {
            gen.push_rendered_expr(&Expr::Block(ExprBlock {
                attrs: vec![],
                label: None,
                block: block.clone(),
            }));
        }
    }
}

impl Generate for NodeText {
    fn generate(&self, gen: &mut Generator) {
        gen.push_escaped_lit(self.value.clone());
    }
}

impl Generate for RawText {
    fn generate(&self, gen: &mut Generator) {
        gen.push_escaped_lit(LitStr::new(&self.to_string_best(), self.span()));
    }
}
