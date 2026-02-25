use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, spanned::Spanned};

use crate::{
    AttributeValueNode, ComponentInstantiationMode, Context, Document, Maud, Nodes, Rsx,
    html::{self, generate::Generator},
};

#[allow(clippy::needless_pass_by_value)]
pub fn renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    match (renderable_element(&input), attribute_renderable(&input)) {
        (Ok(None), Ok(None)) => Err(Error::new(
            Span::call_site(),
            "expected at least one of `maud`, `rsx`, or `attribute` attributes",
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

fn renderable_element(input: &DeriveInput) -> syn::Result<Option<TokenStream>> {
    let mut attrs = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("maud") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Maud>>
                        as fn(
                            TokenStream,
                            bool,
                            Option<ComponentInstantiationMode>,
                        ) -> syn::Result<TokenStream>,
                    Some(ComponentInstantiationMode::StructLiteral),
                ))
            } else if attr.path().is_ident("maud_cb") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Maud>>
                        as fn(
                            TokenStream,
                            bool,
                            Option<ComponentInstantiationMode>,
                        ) -> syn::Result<TokenStream>,
                    Some(ComponentInstantiationMode::Builder),
                ))
            } else if attr.path().is_ident("rsx") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Rsx>>
                        as fn(
                            TokenStream,
                            bool,
                            Option<ComponentInstantiationMode>,
                        ) -> syn::Result<TokenStream>,
                    Some(ComponentInstantiationMode::StructLiteral),
                ))
            } else if attr.path().is_ident("rsx_cb") {
                Some((
                    attr,
                    html::generate::lazy::<Document<Rsx>>
                        as fn(
                            TokenStream,
                            bool,
                            Option<ComponentInstantiationMode>,
                        ) -> syn::Result<TokenStream>,
                    Some(ComponentInstantiationMode::Builder),
                ))
            } else {
                None
            }
        })
        .peekable();

    let (lazy_fn, tokens, instantiation_mode) = match (attrs.next(), attrs.peek()) {
        (Some((attr, f, mode)), None) => (f, attr.meta.require_list()?.tokens.clone(), mode),
        (Some((attr, _, _)), Some(_)) => {
            let mut error = Error::new(
                attr.span(),
                "cannot have multiple `maud` or `rsx` attributes",
            );
            for (attr, _, _) in attrs {
                error.combine(syn::Error::new(
                    attr.span(),
                    "cannot have multiple `maud` or `rsx` attributes",
                ));
            }
            return Err(error);
        }
        (None, _) => {
            return Ok(None);
        }
    };

    let lazy = lazy_fn(tokens, true, instantiation_mode)?;

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable for #name #ty_generics #where_clause {
            fn render_to(&self, #buffer_ident: &mut ::hypertext::Buffer) {
                ::hypertext::Renderable::render_to(&#lazy, #buffer_ident);
            }
        }
    };
    Ok(Some(output))
}

fn attribute_renderable(input: &DeriveInput) -> syn::Result<Option<TokenStream>> {
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
            return Ok(None);
        }
    };

    let lazy = html::generate::lazy::<Nodes<AttributeValueNode>>(tokens, true, None)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let context_marker = Context::AttributeValue.marker_type();
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable<#context_marker> for #name #ty_generics
            #where_clause {
            fn render_to(
                &self,
                #buffer_ident: &mut ::hypertext::AttributeBuffer,
            ) {
                ::hypertext::Renderable::render_to(
                    &#lazy,
                    #buffer_ident,
                );
            }
        }
    };

    Ok(Some(output))
}
