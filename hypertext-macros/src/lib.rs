#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use proc_macro2::{Ident, Span};
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;

mod generate;
mod maud;
mod rstml;

#[proc_macro]
pub fn maud(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let len_estimate = tokens.to_string().len();
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    maud::parse(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |markup| generate::normal(len_estimate, output_ident, markup),
        )
        .into()
}

#[proc_macro]
pub fn maud_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    maud::parse(tokens.into())
        .map_or_else(
            |err| err.to_compile_error(),
            |markup| generate::r#static(output_ident, markup),
        )
        .into()
}

#[proc_macro]
pub fn rsx(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let len_estimate = tokens.to_string().len();
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    let (nodes, diagnostics) = rstml::parse(tokens.into());
    let output = generate::normal(len_estimate, output_ident, nodes);
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
pub fn html_static(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output_ident = Ident::new("hypertext_output", Span::mixed_site());

    let (nodes, diagnostics) = rstml::parse(tokens.into());
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
