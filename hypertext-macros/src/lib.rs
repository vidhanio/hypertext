#![forbid(unsafe_code)]
#![allow(missing_docs, clippy::large_enum_variant)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use proc_macro2::{Ident, Span};
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;

mod generate;
#[cfg(feature = "maud")]
mod maud;
#[cfg(feature = "rsx")]
mod rsx;

#[proc_macro]
#[cfg(feature = "maud")]
pub fn maud_closure(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let len_estimate = tokens.to_string().len();

    maud::parse(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |markup| generate::lazy(markup, len_estimate),
        )
        .into()
}

#[proc_macro]
#[cfg(feature = "maud")]
pub fn maud_literal(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    maud::parse(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |markup| generate::r#static(output_ident, markup),
        )
        .into()
}

#[proc_macro]
#[cfg(feature = "rsx")]
pub fn rsx_closure(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let len_estimate = tokens.to_string().len();

    let (nodes, diagnostics) = rsx::parse(tokens.into());
    let output = generate::lazy(nodes, len_estimate);
    let diagnostics = diagnostics.into_iter().map(Diagnostic::emit_as_expr_tokens);

    quote! {
        {
            #(#diagnostics;)*
            #output
        }
    }
    .into()
}

#[proc_macro]
#[cfg(feature = "rsx")]
pub fn rsx_literal(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    let (nodes, diagnostics) = rsx::parse(tokens.into());
    let output = generate::r#static(output_ident, nodes);
    let diagnostics = diagnostics.into_iter().map(Diagnostic::emit_as_expr_tokens);

    quote! {
        {
            #(#diagnostics;)*
            #output
        }
    }
    .into()
}
