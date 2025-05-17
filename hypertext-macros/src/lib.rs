#![expect(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod component;
mod generate;
mod maud;
mod node;
mod rsx;

use node::{Markup, Syntax};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, DeriveInput, Ident, ItemFn};

use self::{
    component::{extract_fields, ToPascalCase},
    maud::Maud,
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
    Markup<S>: Parse,
{
    let len_estimate = tokens.to_string().len();

    generate::closure::<S>(tokens.into(), len_estimate)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn literal<S: Syntax>(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream
where
    Markup<S>: Parse,
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
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract function details
    let vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_generics = &input_fn.sig.generics;
    let fn_body = &input_fn.block;
    let fn_return_type = &input_fn.sig.output;

    // Convert function name to PascalCase for struct name
    let struct_name_str = fn_name.to_string().to_pascal_case();
    let struct_name = syn::Ident::new(&struct_name_str, fn_name.span());

    // Extract function parameters to use as struct fields
    let fields = extract_fields(&input_fn.sig);

    // Extract the field identifiers and types separately for different uses
    let field_names: Vec<&Ident> = fields.iter().map(|(name, _)| name).collect();
    // let field_types: Vec<&Type> = fields.iter().map(|(_, ty)| ty).collect();

    // Create parameter list for function definition - collect into Vec to avoid move
    let fn_params: Vec<TokenStream2> = fields
        .iter()
        .map(|(name, ty)| {
            quote! { #name: #ty }
        })
        .collect();

    // Generate field declarations - collect into Vec
    let field_declarations: Vec<TokenStream2> = fields
        .iter()
        .map(|(name, ty)| {
            quote! { pub #name: #ty }
        })
        .collect();

    // Generate the internal function call parameters - collect into Vec
    let fn_call_params: Vec<TokenStream2> = field_names
        .iter()
        .map(|&name| {
            quote! { #name }
        })
        .collect();

    // Extract generic parameters and constraints
    let (impl_generics, ty_generics, where_clause) = fn_generics.split_for_impl();

    // Generate the output
    let output = quote! {
        // Create a struct with the fields
        #vis struct #struct_name #ty_generics {
            #(#field_declarations),*
        }

        // Implement Renderable trait
        impl #impl_generics Renderable for #struct_name #ty_generics #where_clause {
            fn render_to(&self, output: &mut String) {
                // Define the original function inside the render_to method
                fn renderable_fn #impl_generics(#(#fn_params),*) #fn_return_type #where_clause #fn_body

                // Destructure self to get the fields
                let Self { #(#field_names),* } = self;

                // Call the function with the struct fields and render the result
                renderable_fn(#(#fn_call_params.clone()),*).render_to(output);
            }
        }

        // Provide a constructor function with the original function name
        #vis fn #fn_name #impl_generics(#(#fn_params),*) -> #struct_name #ty_generics #where_clause {
            #struct_name {
                #(#field_names),*
            }
        }
    };

    output.into()
}
