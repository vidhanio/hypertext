use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, spanned::Spanned};

use crate::{AttributeValueNode, Document, Maud, Nodes, Rsx, html};

pub fn renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut attrs = input
        .attrs
        .into_iter()
        .filter_map(|attr| {
            if attr.path().is_ident("maud") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Maud>>
                        as fn(TokenStream, bool, &str) -> syn::Result<TokenStream>,
                ))
            } else if attr.path().is_ident("rsx") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Rsx>>
                        as fn(TokenStream, bool, &str) -> syn::Result<TokenStream>,
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
                "cannot have multiple `maud` or `rsx` attributes",
            );
            for (attr, _) in attrs {
                error.combine(syn::Error::new(
                    attr.span(),
                    "cannot have multiple `maud` or `rsx` attributes",
                ));
            }
            return Err(error);
        }
        (None, _) => {
            return Err(Error::new(
                Span::call_site(),
                "missing `maud` or `rsx` attribute",
            ));
        }
    };

    let lazy = lazy_fn(tokens, true, "Lazy")?;

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let output = quote! {
        impl #impl_generics ::hypertext::Renderable for #name #ty_generics #where_clause {
            fn render_to(&self, output: &mut ::hypertext::String) {
                ::hypertext::Renderable::render_to(&#lazy, output);
            }
        }
    };
    Ok(output)
}

pub fn attribute_renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    let mut attrs = input
        .attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident("attribute"))
        .peekable();

    let tokens = match (attrs.next(), attrs.peek()) {
        (Some(attr), None) => attr.meta.require_list()?.tokens.clone(),
        (Some(_), Some(_)) => {
            let mut error = Error::new(
                Span::call_site(),
                "cannot have multiple `attribute` attributes",
            );
            for attr in attrs {
                error.combine(syn::Error::new(
                    attr.span(),
                    "cannot have multiple `attribute` attributes",
                ));
            }
            return Err(error);
        }
        (None, _) => {
            return Err(Error::new(
                Span::call_site(),
                "missing `attribute` attribute",
            ));
        }
    };

    let lazy = html::generate::lazy::<Nodes<AttributeValueNode>>(tokens, true, "LazyAttribute")?;
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let output = quote! {
        impl #impl_generics ::hypertext::AttributeRenderable for #name #ty_generics
            #where_clause {
            fn render_attribute_to(
                &self,
                output: &mut ::hypertext::String,
            ) {
                ::hypertext::AttributeRenderable::render_attribute_to(
                    &#lazy,
                    output,
                );
            }
        }
    };

    Ok(output)
}
