#![expect(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod generate;
mod maud;
mod node;
mod rsx;

use node::{Markup, Syntax};
use syn::parse::Parse;

use self::{maud::Maud, rsx::Rsx};

#[proc_macro]
pub fn maud_closure(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    closure::<Maud>(tokens)
}

#[proc_macro]
pub fn maud_literal(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    literal::<Maud>(tokens)
}

#[proc_macro]
pub fn rsx_closure(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    closure::<Rsx>(tokens)
}

#[proc_macro]
pub fn rsx_literal(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    literal::<Rsx>(tokens)
}

fn closure<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Markup<S>: Parse,
{
    let len_estimate = tokens.to_string().len();

    generate::closure::<S>(tokens.into(), len_estimate)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn literal<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Markup<S>: Parse,
{
    generate::literal::<S>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
