use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, Ident, spanned::Spanned};

use crate::html::{
    AttributeValue, Context, Maud, Rsx,
    generate::{Config, Generator, NodeFlavour, Semantics},
};

#[allow(clippy::needless_pass_by_value)]
pub fn renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    let node = renderable_node_attr(&input)?;

    match (
        renderable_node(&input, node.as_ref()),
        renderable_attribute(&input),
    ) {
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

struct RenderableNode {
    flavour: NodeFlavour,
}

impl RenderableNode {
    fn marker_type(&self) -> TokenStream {
        match self.flavour {
            NodeFlavour::Html => quote!(::hypertext::context::Node),
            NodeFlavour::Xml(crate::html::generate::XmlFlavour::Svg) => {
                quote!(
                    ::hypertext::context::Node<
                        ::hypertext::context::Xml<::hypertext::context::Svg>,
                    >
                )
            }
            NodeFlavour::Xml(crate::html::generate::XmlFlavour::MathMl) => {
                quote!(
                    ::hypertext::context::Node<
                        ::hypertext::context::Xml<::hypertext::context::MathMl>,
                    >
                )
            }
        }
    }
}

fn renderable_node_attr(input: &DeriveInput) -> syn::Result<Option<RenderableNode>> {
    let mut attrs = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("renderable"));

    let Some(attr) = attrs.next() else {
        return Ok(None);
    };

    if let Some(extra_attr) = attrs.next() {
        return Err(Error::new(
            extra_attr.span(),
            "cannot have multiple `#[renderable(...)]` attributes",
        ));
    }

    let mut node = None;
    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("node") {
            node = Some(meta.value()?.parse::<Ident>()?);
            return Ok(());
        }

        Err(meta.error("unsupported `#[renderable(...)]` parameter"))
    })?;

    let Some(node) = node else {
        return Err(Error::new(
            attr.span(),
            "expected `#[renderable(node = html|svg|mathml)]`",
        ));
    };

    Ok(Some(RenderableNode {
        flavour: node_flavour_from_ident(&node)?,
    }))
}

fn node_flavour_from_ident(ident: &Ident) -> syn::Result<NodeFlavour> {
    match ident.to_string().as_str() {
        "html" => Ok(NodeFlavour::Html),
        "svg" => Ok(NodeFlavour::Xml(crate::html::generate::XmlFlavour::Svg)),
        "mathml" => Ok(NodeFlavour::Xml(crate::html::generate::XmlFlavour::MathMl)),
        _ => Err(Error::new_spanned(
            ident,
            "renderable node must be one of `html`, `svg`, or `mathml`",
        )),
    }
}

fn renderable_node(
    input: &DeriveInput,
    node: Option<&RenderableNode>,
) -> syn::Result<Option<TokenStream>> {
    enum SyntaxKind {
        Maud,
        Rsx,
    }

    let mut attrs = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("maud") {
                Some((attr, SyntaxKind::Maud))
            } else if attr.path().is_ident("rsx") {
                Some((attr, SyntaxKind::Rsx))
            } else {
                None
            }
        })
        .peekable();

    let flavour = node.map_or(NodeFlavour::Html, |node| node.flavour);

    let (syntax, tokens) = match (attrs.next(), attrs.peek()) {
        (Some((attr, syntax)), None) => (syntax, attr.meta.require_list()?.tokens.clone()),
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
            if node.is_some() {
                return Err(Error::new(
                    input.span(),
                    "`#[renderable(node = ...)]` may only be used together with `#[maud(...)]` or `#[rsx(...)]`",
                ));
            }
            return Ok(None);
        }
    };

    let lazy = match syntax {
        SyntaxKind::Maud => Config {
            lazy: Some(Semantics::Move),
        }
        .generate_nodes::<Maud>(flavour, tokens)?,
        SyntaxKind::Rsx => Config {
            lazy: Some(Semantics::Move),
        }
        .generate_nodes::<Rsx>(flavour, tokens)?,
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let context_ty = node.map_or_else(
        || syn::parse_quote!(::hypertext::context::Node),
        RenderableNode::marker_type,
    );
    let output = quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable<#context_ty> for #name #ty_generics #where_clause {
            fn render_to(&self, #buffer_ident: &mut ::hypertext::Buffer<#context_ty>) {
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

    let lazy = Config {
        lazy: Some(Semantics::Move),
    }
    .generate_attrs(tokens)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let buffer_ident = Generator::buffer_ident();
    let ctx = AttributeValue::marker_type(NodeFlavour::Html);
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

                            Err(meta.error("unexpected param for `#[builder(...)]`"))
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
                <Self as ::core::default::Default>::default()
            }

            #vis fn build(self) -> Self {
                self
            }

            #(#methods)*
        }
    };

    Ok(output)
}
