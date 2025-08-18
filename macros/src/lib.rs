#![expect(missing_docs)]

mod derive;
mod html;
mod renderable;

use html::{AttributeValue, Many};
use proc_macro::TokenStream;
use syn::{DeriveInput, ItemFn, parse::Parse, parse_macro_input};

use self::html::{Document, Maud, Rsx, Syntax};
use crate::renderable::RenderableArgs;

#[proc_macro]
pub fn maud(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lazy::<Maud>(tokens, true)
}

#[proc_macro]
pub fn maud_borrow(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lazy::<Maud>(tokens, false)
}

#[proc_macro]
pub fn rsx(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lazy::<Rsx>(tokens, true)
}

#[proc_macro]
pub fn rsx_borrow(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    lazy::<Rsx>(tokens, false)
}

#[proc_macro]
pub fn maud_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_::<Maud>(tokens)
}

#[proc_macro]
pub fn rsx_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_::<Rsx>(tokens)
}

fn lazy<S: Syntax>(tokens: proc_macro::TokenStream, move_: bool) -> proc_macro::TokenStream
where
    Document<S>: Parse,
{
    html::generate::lazy::<Document<S>>(tokens.into(), move_)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn static_<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Document<S>: Parse,
{
    html::generate::literal::<Document<S>>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn attribute(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute_lazy(tokens, true)
}

#[proc_macro]
pub fn attribute_borrow(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    attribute_lazy(tokens, false)
}

fn attribute_lazy(tokens: proc_macro::TokenStream, move_: bool) -> proc_macro::TokenStream {
    html::generate::lazy::<Many<AttributeValue>>(tokens.into(), move_)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn attribute_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html::generate::literal::<Many<AttributeValue>>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Renderable, attributes(maud, rsx, attribute))]
pub fn derive_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::renderable(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as RenderableArgs);
    let item = parse_macro_input!(item as ItemFn);

    renderable::generate(attr, &item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
