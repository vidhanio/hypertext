#![expect(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod component;
mod derive;
mod html;

use html::{AttributeValueNode, Nodes};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, ItemFn,
    parse::{Nothing, Parse},
    parse_macro_input,
};

use self::html::{Document, Maud, Rsx, Syntax};

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
    html::generate::lazy::<Document<S>>(tokens.into(), move_, "Lazy")
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn static_<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Document<S>: Parse,
{
    html::generate::literal::<Document<S>>(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |lit| quote!(::hypertext::Raw(#lit)),
        )
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
    html::generate::lazy::<Nodes<AttributeValueNode>>(tokens.into(), move_, "LazyAttribute")
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn attribute_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    html::generate::literal::<Nodes<AttributeValueNode>>(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |lit| quote!(::hypertext::RawAttribute(#lit)),
        )
        .into()
}

#[proc_macro_derive(Renderable, attributes(maud, rsx))]
pub fn derive_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::renderable(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(AttributeRenderable, attributes(attribute))]
pub fn derive_attribute_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::attribute_renderable(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    let item = parse_macro_input!(item as ItemFn);

    component::generate(&item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
