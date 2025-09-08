use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, spanned::Spanned};

use crate::{
    AttributeValue, Document, Many, Maud, Rsx,
    html::{self, Context, generate::Generator},
};

#[allow(clippy::needless_pass_by_value)]
pub fn renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    match (renderable_node(&input), renderable_attribute(&input)) {
        (Ok(None), Ok(None)) => Err(Error::new(
            Span::call_site(),
            "expected at least one of `#[maud(...)]`, `#[rsx(...)]`, or `#[attribute(...)]`",
        )),
        (Ok(element), Ok(attribute)) => Ok(quote! {
            #element
            #attribute
        }),
        (Ok(_), Err(e)) | (Err(e), Ok(_)) => Err(e),
        (Err(mut e1), Err(e2)) => {
            e1.combine(e2);
            Err(e1)
        }
    }
}

fn renderable_node(input: &DeriveInput) -> syn::Result<Option<TokenStream>> {
    let mut attrs = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("maud") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Maud>>
                        as fn(TokenStream, bool) -> syn::Result<TokenStream>,
                ))
            } else if attr.path().is_ident("rsx") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Rsx>>
                        as fn(TokenStream, bool) -> syn::Result<TokenStream>,
                ))
            } else {
                None
            }
        })
        .peekable();

    let (lazy_fn, tokens) = match (attrs.next(), attrs.peek()) {
        (Some((attr, f)), None) => (f, attr.meta.require_list()?.tokens.clone()),
        (Some((attr, _)), Some(_)) => {
            let mut error = Error::new(
                attr.span(),
                "cannot have multiple `#[maud(...)]` or `#[rsx(...)]` attributes",
            );
            for (attr, _) in attrs {
                error.combine(Error::new(
                    attr.span(),
                    "cannot have multiple `#[maud(...)]` or `#[rsx(...)]` attributes",
                ));
            }
            return Err(error);
        }
        (None, _) => {
            return Ok(None);
        }
    };

    let lazy = lazy_fn(tokens, true)?;

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable for #name #ty_generics #where_clause {
            fn render_to(&self, #buffer_ident: &mut ::hypertext::Buffer) {
                #buffer_ident.push(#lazy);
            }
        }
    };
    Ok(Some(output))
}

fn renderable_attribute(input: &DeriveInput) -> syn::Result<Option<TokenStream>> {
    let mut attrs = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("attribute"))
        .peekable();

    let tokens = match (attrs.next(), attrs.peek()) {
        (Some(attr), None) => attr.meta.require_list()?.tokens.clone(),
        (Some(_), Some(_)) => {
            let mut error = Error::new(
                Span::call_site(),
                "cannot have multiple `#[attribute(...)]` attributes",
            );
            for attr in attrs {
                error.combine(Error::new(
                    attr.span(),
                    "cannot have multiple `#[attribute(...)]` attributes",
                ));
            }
            return Err(error);
        }
        (None, _) => {
            return Ok(None);
        }
    };

    let lazy = html::generate::lazy::<Many<AttributeValue>>(tokens, true)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let ctx = AttributeValue::marker_type();
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable<#ctx> for #name #ty_generics
            #where_clause {
            fn render_to(
                &self,
                #buffer_ident: &mut ::hypertext::AttributeBuffer,
            ) {
                #buffer_ident.push(#lazy);
            }
        }
    };

    Ok(Some(output))
}
