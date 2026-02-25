use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, spanned::Spanned};

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

#[allow(clippy::needless_pass_by_value)]
pub fn builder(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "#[derive(Builder)] may only be used on structs",
        ));
    };

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut methods = Vec::new();
    for field in &data_struct.fields {
        if let Some(name) = &field.ident {
            let vis = &field.vis;
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

    let output = if methods.is_empty() {
        quote! {}
    } else {
        quote! {
            #[automatically_derived]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#methods)*
            }
        }
    };

    Ok(output)
}
