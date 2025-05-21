use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, Pat, PatType, Type};

pub fn generate(fn_item: &ItemFn) -> syn::Result<TokenStream> {
    let vis = &fn_item.vis;

    let mut fields = Vec::new();
    let mut field_names = Vec::new();
    let mut field_refs = Vec::new();

    for input in &fn_item.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            let ident = match &**pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return Err(syn::Error::new_spanned(
                        pat,
                        "component function parameters must be identifiers",
                    ));
                }
            };
            let (ty, ref_token) = match &**ty {
                Type::Reference(ty_ref) => {
                    if ty_ref.mutability.is_some() {
                        return Err(syn::Error::new_spanned(
                            ty_ref,
                            "component function parameters cannot be mutable references",
                        ));
                    }

                    if ty_ref.lifetime.is_some() {
                        (ty, None)
                    } else {
                        (&ty_ref.elem, Some(ty_ref.and_token))
                    }
                }
                _ => (ty, None),
            };
            fields.push(quote! {
                #vis #ident: #ty
            });
            field_names.push(ident.clone());
            field_refs.push(ref_token);
        } else {
            return Err(syn::Error::new_spanned(
                input,
                "component function parameters do not support `self` or `&self`",
            ));
        }
    }

    let fn_name = &fn_item.sig.ident;

    let struct_name_str = to_pascal_case(&fn_name.to_string());
    let struct_name = Ident::new(&struct_name_str, fn_name.span());

    let (impl_generics, ty_generics, where_clause) = fn_item.sig.generics.split_for_impl();

    let output = quote! {
        #[allow(clippy::needless_lifetimes)]
        #fn_item

        #vis struct #struct_name #ty_generics {
            #(#fields),*
        }

        impl #impl_generics ::hypertext::Renderable for #struct_name #ty_generics #where_clause {
            fn render_to(&self, output: &mut ::hypertext::proc_macros::String) {
                ::hypertext::Renderable::render_to(
                    &#fn_name(#(
                        #field_refs self.#field_names
                    ),*),
                    output,
                );
            }
        }
    };

    Ok(output)
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
