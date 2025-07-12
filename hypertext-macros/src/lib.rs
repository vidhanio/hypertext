#![expect(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod component;
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

#[proc_macro_derive(Renderable)]
pub fn derive_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    quote! {
        impl<#impl_generics> ::hypertext::Renderable for #ident #ty_generics #where_clause {
            fn render_to(&self, output: &mut ::hypertext::String) {
                ::hypertext::Renderable::render_to(
                    &::hypertext::Displayed(self),
                    output,
                )
            }
        }
    }
    .into()
}

#[proc_macro_derive(AttributeRenderable)]
pub fn derive_attribute_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    quote! {
        impl<#impl_generics> ::hypertext::AttributeRenderable for #ident #ty_generics #where_clause {
            fn render_attribute_to(
                &self,
                output: &mut ::hypertext::String,
            ) {
                ::hypertext::AttributeRenderable::render_attribute_to(
                    &::hypertext::Displayed(self),
                    output,
                )
            }
        }
    }
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
