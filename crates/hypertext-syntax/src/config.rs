use syn::{
    AttrStyle, Attribute, Error, LitBool, MetaList, bracketed, custom_keyword,
    parse::{Parse, ParseStream},
};

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct Config {
    pub syntax: MacroSyntax,
    pub quotes: Quotes,
    pub output: Output,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attribute = {
            let content;
            Attribute {
                pound_token: input.parse()?,
                style: AttrStyle::Inner(input.parse()?),
                bracket_token: bracketed!(content in input),
                meta: content.parse()?,
            }
        };

        if !attribute.path().is_ident("config") {
            return Err(syn::Error::new_spanned(
                &attribute,
                "expected `#![config(...)]` attribute",
            ));
        }

        let mut syntax = None::<MacroSyntax>;
        let mut quotes = None::<Quotes>;
        let mut output = None::<Output>;

        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("syntax") {
                if syntax.is_some() {
                    return Err(meta.error("duplicate `syntax` option"));
                }

                syntax = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("quotes") {
                if quotes.is_some() {
                    return Err(meta.error("duplicate `quotes` option"));
                }

                quotes = Some(meta.value()?.parse()?);
            } else if meta.path.is_ident("output") {
                if output.is_some() {
                    return Err(meta.error("duplicate `output` option"));
                }

                output = Some(meta.value()?.parse()?);
            } else {
                return Err(
                    meta.error("unrecognized option: expected `syntax`, `quotes`, or `output`")
                );
            }

            Ok(())
        })?;

        Ok(Self {
            syntax: syntax.ok_or_else(|| {
                Error::new_spanned(attribute.path(), "expected `syntax` option to be specified")
            })?,
            quotes: quotes.ok_or_else(|| {
                Error::new_spanned(attribute.path(), "expected `quotes` option to be specified")
            })?,
            output: output.ok_or_else(|| {
                Error::new_spanned(attribute.path(), "expected `output` option to be specified")
            })?,
        })
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum MacroSyntax {
    Maud,
    Rsx,
    Attribute,
}

impl Parse for MacroSyntax {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        custom_keyword!(maud);
        custom_keyword!(rsx);
        custom_keyword!(attribute);

        let lookahead = input.lookahead1();

        if lookahead.peek(maud) {
            input.parse::<maud>()?;
            Ok(Self::Maud)
        } else if lookahead.peek(rsx) {
            input.parse::<rsx>()?;
            Ok(Self::Rsx)
        } else if lookahead.peek(attribute) {
            input.parse::<attribute>()?;
            Ok(Self::Attribute)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Quotes {
    Double,
    Single,
}

impl Parse for Quotes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        custom_keyword!(double);
        custom_keyword!(single);

        let lookahead = input.lookahead1();

        if lookahead.peek(double) {
            input.parse::<double>()?;
            Ok(Self::Double)
        } else if lookahead.peek(single) {
            input.parse::<single>()?;
            Ok(Self::Single)
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub enum Output {
    Simple,
    Lazy { move_: bool },
}

impl Parse for Output {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        custom_keyword!(simple);
        custom_keyword!(lazy);

        let lookahead = input.lookahead1();

        if lookahead.peek(simple) {
            input.parse::<simple>()?;
            Ok(Self::Simple)
        } else if lookahead.peek(lazy) {
            let list = input.parse::<MetaList>()?;

            if !list.path.is_ident("lazy") {
                return Err(Error::new_spanned(&list.path, "expected `lazy(...)`"));
            }

            let mut move_ = None::<bool>;

            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("move") {
                    if move_.is_some() {
                        return Err(meta.error("duplicate `move` option"));
                    }

                    move_ = Some(meta.value()?.parse::<LitBool>()?.value);
                } else {
                    return Err(meta.error("unrecognized option: expected `move`"));
                }

                Ok(())
            })?;

            Ok(Self::Lazy {
                move_: move_.ok_or_else(|| {
                    Error::new_spanned(&list.path, "expected `move` option to be specified")
                })?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn works() {
        let attr = quote! {
            #![config(
                syntax = maud,
                quotes = double,
                output = lazy(move = false)
            )]
        };

        assert_eq!(
            syn::parse2::<Config>(attr).unwrap(),
            Config {
                syntax: MacroSyntax::Maud,
                quotes: Quotes::Double,
                output: Output::Lazy { move_: false }
            }
        );
    }
}
