use std::fmt::{self, Display, Formatter, Write};

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    Error, Ident, LitBool, LitChar, LitFloat, LitInt, LitStr, Token,
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
                NameFragment::Int(num) => {
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

    pub fn parse_attr_value(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let mut name = Vec::new();

        if lookahead.peek(Ident::peek_any) || lookahead.peek(LitInt) {
            name.push(input.parse()?);

            while input.peek(Token![-])
                || input.peek(Token![:])
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
    Int(LitInt),
    Hyphen(Token![-]),
    Colon(Token![:]),
    Dot(Token![.]),
}

impl NameFragment {
    fn span(&self) -> Span {
        match self {
            Self::Ident(ident) => ident.span(),
            Self::Int(int) => int.span(),
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

        if lookahead.peek(Token![-]) {
            input.parse().map(Self::Hyphen)
        } else if lookahead.peek(Token![:]) {
            input.parse().map(Self::Colon)
        } else if lookahead.peek(Token![.]) {
            input.parse().map(Self::Dot)
        } else if lookahead.peek(Ident::peek_any) {
            input.call(Ident::parse_any).map(Self::Ident)
        } else if lookahead.peek(LitInt) {
            input.parse().map(Self::Int)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Display for NameFragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(ident) => write!(f, "{ident}"),
            Self::Int(num) => write!(f, "{num}"),
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
    Char(LitChar),
}

impl Literal {
    pub fn lit_str(&self) -> LitStr {
        match self {
            Self::Str(lit) => lit.clone(),
            Self::Int(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Bool(lit) => LitStr::new(&lit.value.to_string(), lit.span()),
            Self::Float(lit) => LitStr::new(&lit.to_string(), lit.span()),
            Self::Char(lit) => LitStr::new(&lit.value().to_string(), lit.span()),
        }
    }
}

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            let lit = input.parse::<LitStr>()?;
            if !lit.suffix().is_empty() {
                let suffix = lit.suffix();
                let next_quote = if input.peek(LitStr) { r#"\""# } else { "" };
                return Err(Error::new_spanned(
                    &lit,
                    format!(
                        r#"string suffixes are not allowed in literals (you probably meant `"...\"{suffix}{next_quote}..."` or `"..." {suffix}`)"#,
                    ),
                ));
            }
            let value = unindent(&lit.value());
            Ok(Self::Str(LitStr::new(&value, lit.span())))
        } else if lookahead.peek(LitInt) {
            let lit = input.parse::<LitInt>()?;
            if !lit.suffix().is_empty() {
                return Err(Error::new_spanned(
                    &lit,
                    "integer literals cannot have suffixes",
                ));
            }
            Ok(Self::Int(lit))
        } else if lookahead.peek(LitBool) {
            input.parse().map(Self::Bool)
        } else if lookahead.peek(LitFloat) {
            let lit = input.parse::<LitFloat>()?;
            if !lit.suffix().is_empty() {
                return Err(Error::new_spanned(
                    &lit,
                    "float literals cannot have suffixes",
                ));
            }
            Ok(Self::Float(lit))
        } else if lookahead.peek(LitChar) {
            let lit = input.parse::<LitChar>()?;
            if !lit.suffix().is_empty() {
                return Err(Error::new_spanned(
                    &lit,
                    "character literals cannot have suffixes",
                ));
            }
            Ok(Self::Char(lit))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Str(lit) => lit.to_tokens(tokens),
            Self::Int(lit) => lit.to_tokens(tokens),
            Self::Bool(lit) => lit.to_tokens(tokens),
            Self::Float(lit) => lit.to_tokens(tokens),
            Self::Char(lit) => lit.to_tokens(tokens),
        }
    }
}

// from dtolnay/unindent
fn unindent(s: &str) -> String {
    const fn is_indent(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    let mut lines = s.lines().collect::<Vec<_>>();

    // lines() does not include the last line if it ends with a newline
    if s.ends_with('\n') {
        lines.push("");
    }

    let last_line = lines.len().saturating_sub(1);

    let spaces = lines
        .iter()
        .skip(1) // skip same line as opening quote
        .filter_map(|line| line.chars().position(|ch| !is_indent(ch)))
        .min()
        .unwrap_or_default();

    let mut result = String::with_capacity(s.len());
    for (i, line) in lines.iter().enumerate() {
        if (i == 1 && !lines[0].is_empty())
            || (1 < i && i < last_line)
            || (i == last_line
                && last_line != 0
                && (!line.chars().all(is_indent) || line.is_empty()))
        {
            result.push('\n');
        }
        if i == 0 {
            // Do not un-indent anything on same line as opening quote
            result.push_str(line);
        } else if line.len() > spaces {
            // Whitespace-only lines may have fewer than the number of spaces
            // being removed
            result.push_str(&line[spaces..]);
        }
    }
    result
}
