use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, spanned::Spanned};

use crate::{
    AttributeValue, Config, Document, Many, Maud, Rsx, Semantics,
    html::{Context, generate::Generator},
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
                    (|tokens| Config::Lazy(Semantics::Move).generate::<Document<Maud>>(tokens))
                        as fn(_) -> _,
                ))
            } else if attr.path().is_ident("rsx") {
                Some((
                    attr,
                    (|tokens| Config::Lazy(Semantics::Move).generate::<Document<Rsx>>(tokens))
                        as fn(_) -> _,
                ))
            } else {
                None
            }
        })
        .peekable();

    let (generate_fn, tokens) = match (attrs.next(), attrs.peek()) {
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

    let lazy = generate_fn(tokens)?;

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

    let lazy = Config::Lazy(Semantics::Move).generate::<Many<AttributeValue>>(tokens)?;
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

#[allow(clippy::needless_pass_by_value)]
pub fn default_builder(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(Error::new(
            input.span(),
            "#[derive(DefaultBuilder)] may only be used on structs",
        ));
    };

    let struct_name = &input.ident;
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut methods = Vec::new();
    for field in &data_struct.fields {
        if let Some(name) = &field.ident {
            let ty = &field.ty;

            let is_skipped = field
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("builder"))
                .map_or(Ok(false), |builder_attr| {
                    builder_attr
                        .parse_nested_meta(|meta| {
                            if meta.path.is_ident("skip") {
                                return Ok(());
                            }

                            Err(meta.error("unrecognized builder"))
                        })
                        .map(|()| true)
                })?;

            if !is_skipped {
                methods.push(quote! {
                    #[must_use]
                    #vis fn #name(mut self, #name: #ty) -> Self {
                        self.#name = #name;
                        self
                    }
                });
            }
        }
    }

    let output = quote! {
        #[automatically_derived]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #vis fn builder() -> Self {
                Self::default()
            }

            #vis fn build(self) -> Self {
                self
            }

            #(#methods)*
        }
    };

    Ok(output)
}
