use proc_macro2::{Span, TokenStream, TokenTree};
use syn::Lit;

#[derive(Debug)]
pub enum Markup {
    /// Used as a placeholder value on parse error.
    ParseError {
        span: Span,
    },
    Block(Block),
    Literal {
        content: String,
        span: Span,
    },
    Symbol {
        symbol: TokenStream,
    },
    Splice {
        expr: TokenStream,
        outer_span: Span,
    },
    Element {
        name: TokenStream,
        attrs: Vec<Attr>,
        body: ElementBody,
    },
    Let {
        at_span: Span,
        tokens: TokenStream,
    },
    Special {
        segments: Vec<Special>,
    },
    Match {
        at_span: Span,
        head: TokenStream,
        arms: Vec<MatchArm>,
        arms_span: Span,
    },
}

impl Markup {
    pub fn span(&self) -> Span {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::ParseError { span } => span,
            Self::Block(ref block) => block.span(),
            Self::Literal { span, .. } => span,
            Self::Symbol { ref symbol } => span_tokens(symbol.clone()),
            Self::Splice { outer_span, .. } => outer_span,
            Self::Element {
                ref name, ref body, ..
            } => {
                let name_span = span_tokens(name.clone());
                name_span.join_range(body.span())
            }
            Self::Let {
                at_span,
                ref tokens,
            } => at_span.join_range(span_tokens(tokens.clone())),
            Self::Special { ref segments } => join_ranges(segments.iter().map(Special::span)),
            Self::Match {
                at_span, arms_span, ..
            } => at_span.join_range(arms_span),
        }
    }
}

#[derive(Debug)]
pub enum Attr {
    Class {
        dot_span: Span,
        name: Markup,
        toggler: Option<Toggler>,
    },
    Id {
        hash_span: Span,
        name: Markup,
    },
    Named {
        named_attr: NamedAttr,
    },
}

impl Attr {
    pub fn span(&self) -> Span {
        match *self {
            Self::Class {
                dot_span,
                ref name,
                ref toggler,
            } => {
                let name_span = name.span();
                let dot_name_span = dot_span.join_range(name_span);
                toggler.as_ref().map_or(dot_name_span, |toggler| {
                    dot_name_span.join_range(toggler.cond_span)
                })
            }
            Self::Id {
                hash_span,
                ref name,
            } => {
                let name_span = name.span();
                hash_span.join_range(name_span)
            }
            Self::Named { ref named_attr } => named_attr.span(),
        }
    }
}

#[derive(Debug)]
pub enum ElementBody {
    Void { semi_span: Span },
    Block { block: Block },
}

impl ElementBody {
    pub fn span(&self) -> Span {
        match *self {
            Self::Void { semi_span } => semi_span,
            Self::Block { ref block } => block.span(),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub markups: Vec<Markup>,
    pub outer_span: Span,
}

impl Block {
    pub const fn span(&self) -> Span {
        self.outer_span
    }
}

#[derive(Debug)]
pub struct Special {
    pub at_span: Span,
    pub head: TokenStream,
    pub body: Block,
}

impl Special {
    pub fn span(&self) -> Span {
        let body_span = self.body.span();
        self.at_span.join_range(body_span)
    }
}

#[derive(Debug)]
pub struct NamedAttr {
    pub name: TokenStream,
    pub attr_type: AttrType,
}

impl NamedAttr {
    fn span(&self) -> Span {
        let name_span = span_tokens(self.name.clone());
        self.attr_type.span().map_or(name_span, |attr_type_span| {
            name_span.join_range(attr_type_span)
        })
    }
}

#[derive(Debug)]
pub enum AttrType {
    Normal { value: Markup },
    Optional { toggler: Toggler },
    Empty { toggler: Option<Toggler> },
}

impl AttrType {
    fn span(&self) -> Option<Span> {
        match *self {
            AttrType::Normal { ref value } => Some(value.span()),
            AttrType::Optional { ref toggler } => Some(toggler.span()),
            AttrType::Empty { ref toggler } => toggler.as_ref().map(Toggler::span),
        }
    }
}

#[derive(Debug)]
pub struct Toggler {
    pub cond: TokenStream,
    pub cond_span: Span,
}

impl Toggler {
    fn span(&self) -> Span {
        self.cond_span
    }
}

#[derive(Debug)]
pub struct MatchArm {
    pub head: TokenStream,
    pub body: Block,
}

pub fn span_tokens<I: IntoIterator<Item = TokenTree>>(tokens: I) -> Span {
    join_ranges(tokens.into_iter().map(|t| t.span()))
}

pub fn join_ranges<I: IntoIterator<Item = Span>>(ranges: I) -> Span {
    let mut iter = ranges.into_iter();
    let Some(first) = iter.next() else {
        return Span::call_site();
    };
    let last = iter.last().unwrap_or(first);
    first.join_range(last)
}

pub fn name_to_string(name: TokenStream) -> String {
    name.into_iter()
        .map(|token| {
            if let TokenTree::Literal(literal) = token {
                match Lit::new(literal.clone()) {
                    Lit::Str(str) => str.value(),
                    Lit::Char(char) => char.value().to_string(),
                    Lit::ByteStr(byte) => {
                        String::from_utf8(byte.value()).expect("Invalid utf8 byte")
                    }
                    Lit::Byte(byte) => (byte.value() as char).to_string(),
                    _ => literal.to_string(),
                }
            } else {
                token.to_string()
            }
        })
        .collect()
}

trait JoinRange {
    fn join_range(self, other: Self) -> Self;
}

impl JoinRange for Span {
    fn join_range(self, other: Self) -> Self {
        self.join(other).unwrap_or(self)
    }
}
