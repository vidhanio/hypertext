#![expect(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod component;
mod generate;
mod maud;
mod node;
mod rsx;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, ItemFn,
    parse::{Nothing, Parse},
    parse_macro_input,
};

use self::{
    maud::Maud,
    node::{Document, Syntax},
    rsx::Rsx,
};

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
    Document<S>: Parse,
{
    let len_estimate = tokens.to_string().len();

    generate::closure::<S>(tokens.into(), len_estimate)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn literal<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Document<S>: Parse,
{
    generate::literal::<S>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Renderable)]
pub fn derive_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    quote! {
        const _: () = {
            extern crate alloc;

            impl<#impl_generics> ::hypertext::Renderable for #ident #ty_generics #where_clause {
                fn render_to(&self, output: &mut alloc::string::String) {
                    ::hypertext::Renderable::render_to(
                        &::hypertext::Displayed(self),
                        output,
                    )
                }
            }
        };
    }
    .into()
}

#[proc_macro_derive(AttributeRenderable)]
pub fn derive_attribute_renderable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    quote! {
        const _: () = {
            extern crate alloc;

            impl<#impl_generics> ::hypertext::AttributeRenderable for #ident #ty_generics #where_clause {
                fn render_attribute_to(
                    &self,
                    output: &mut alloc::string::String,
                ) {
                    ::hypertext::AttributeRenderable::render_attribute_to(
                        &::hypertext::Displayed(self),
                        output,
                    )
                }
            }
        };
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
