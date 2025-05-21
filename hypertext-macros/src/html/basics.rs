use std::fmt::{self, Display, Formatter, Write};

use proc_macro2::Span;
use syn::{
    Ident, LitBool, LitFloat, LitInt, LitStr, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[derive(PartialEq, Eq, Clone)]
pub struct UnquotedName(Vec<NameFragment>);

impl UnquotedName {
    pub fn ident_string(&self) -> String {
        let mut s = String::new();

        for fragment in &self.0 {
            match fragment {
                NameFragment::Ident(ident) => {
                    _ = write!(s, "{ident}");
                }
                NameFragment::Number(num) => {
                    _ = write!(s, "{num}");
                }
                NameFragment::Hyphen(_) => {
                    s.push('_');
                }
                NameFragment::Colon(_) | NameFragment::Dot(_) => {
                    unreachable!(
                        "unquoted name idents should only contain identifiers, int literals, and hyphens"
                    );
                }
            }
        }

        if s == "super"
            || s == "self"
            || s == "Self"
            || s == "extern"
            || s == "crate"
            || s == "_"
            || s.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            s.insert(0, '_');
        }

        s
    }

    pub fn is_component(&self) -> bool {
        matches!(
            self.0.as_slice(),
            [NameFragment::Ident(ident)]
                if ident.to_string().chars().next().is_some_and(|c| c.is_ascii_uppercase())
        )
    }

    pub fn spans(&self) -> Vec<Span> {
        let mut spans = Vec::new();

        for fragment in &self.0 {
            spans.push(fragment.span());
        }

        spans
    }

    pub fn lits(&self) -> Vec<LitStr> {
        let mut strs = Vec::new();

        for fragment in &self.0 {
            strs.push(LitStr::new(&fragment.to_string(), fragment.span()));
        }

        strs
    }

    pub fn parse_any(input: ParseStream) -> syn::Result<Self> {
        let mut name = Vec::new();

        while input.peek(Token![-])
            || input.peek(Token![:])
            || input.peek(Token![.])
            || (name.last().is_none_or(NameFragment::is_punct)
                && (input.peek(Ident::peek_any) || input.peek(LitInt)))
        {
            name.push(input.parse()?);
        }

        Ok(Self(name))
    }
}

impl Parse for UnquotedName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let mut name = Vec::new();

        if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            name.push(input.parse()?);

            while input.peek(Token![-])
                || (name.last().is_none_or(NameFragment::is_punct)
                    && (input.peek(Ident::peek_any) || input.peek(LitInt)))
            {
                name.push(input.parse()?);
            }

            Ok(Self(name))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum NameFragment {
    Ident(Ident),
    Number(LitInt),
    Hyphen(Token![-]),
    Colon(Token![:]),
    Dot(Token![.]),
}

impl NameFragment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Number(num) => num.span(),
            Self::Hyphen(hyphen) => hyphen.span(),
            Self::Colon(colon) => colon.span(),
            Self::Dot(dot) => dot.span(),
        }
    }

    const fn is_punct(&self) -> bool {
        matches!(self, Self::Hyphen(_) | Self::Colon(_) | Self::Dot(_))
    }
}

impl Parse for NameFragment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Ident)
        } else if lookahead.peek(LitInt) {
            let int = input.parse::<LitInt>()?;

            if !int.suffix().is_empty() {
                return Err(syn::Error::new_spanned(
                    &int,
                    "integer suffixes are not allowed in names",
                ));
            }

            Ok(Self::Number(int))
        } else if lookahead.peek(Token![-]) {
            input.parse().map(Self::Hyphen)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else if lookahead.peek(Token![.]) {
            input.parse().map(Self::Dot)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Display for NameFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "{ident}"),
            Self::Number(num) => write!(f, "{num}"),
            Self::Hyphen(_) => f.write_str("-"),
            Self::Colon(_) => f.write_str(":"),
            Self::Dot(_) => f.write_str("."),
        }
    }
}

pub enum Literal {
    Str(LitStr),
    Int(LitInt),
    Bool(LitBool),
    Float(LitFloat),
}

impl Literal {
    pub fn lit_str(&self) -> LitStr {
        match self {
            Self::Str(lit) => lit.clone(),
            Self::Int(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Bool(lit) => LitStr::new(&lit.value.to_string(), lit.span()),
            Self::Float(lit) => LitStr::new(&lit.to_string(), lit.span()),
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
        } else if lookahead.peek(LitFloat) {
            input.parse().map(Self::Float)
        } else {
            Err(lookahead.error())
        }
    }
}
