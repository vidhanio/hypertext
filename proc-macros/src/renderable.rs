use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, FnArg, Ident, ItemFn, Pat, PatType, Type, Visibility, parse::Parse};

use crate::html::generate::Generator;

pub struct RenderableArgs {
    visibility: Visibility,
    ident: Option<Ident>,
}

impl Parse for RenderableArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            visibility: input.parse()?,
            ident: if input.peek(Ident) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

#[expect(clippy::needless_pass_by_value)]
pub fn generate(args: RenderableArgs, fn_item: ItemFn) -> syn::Result<TokenStream> {
    let mut fields = Vec::new();
    let mut field_names = Vec::new();
    let mut field_refs = Vec::new();

    let vis = if args.visibility == Visibility::Inherited {
        fn_item.vis.clone()
    } else {
        args.visibility
    };

    for input in &fn_item.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            let ident = match &**pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return Err(Error::new_spanned(
                        pat,
                        "renderable function parameters must be identifiers",
                    ));
                }
            };
            let (ty, ref_token) = match &**ty {
                Type::Reference(ty_ref) => {
                    if ty_ref.mutability.is_some() {
                        return Err(Error::new_spanned(
                            ty_ref,
                            "renderable function parameters cannot be mutable references",
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
            return Err(Error::new_spanned(
                input,
                "renderable function parameters do not support `self` or `&self`",
            ));
        }
    }

    let fn_name = &fn_item.sig.ident;

    let struct_name = args
        .ident
        .unwrap_or_else(|| Ident::new(&to_pascal_case(&fn_name.to_string()), fn_name.span()));

    let (impl_generics, ty_generics, where_clause) = fn_item.sig.generics.split_for_impl();

    let struct_body = if fields.is_empty() {
        quote!(;)
    } else {
        quote! {
            { #(#fields),* }
        }
    };

    let buffer_ident = Generator::buffer_ident();

    let output = quote! {
        #[allow(clippy::needless_lifetimes)]
        #fn_item

        #vis struct #struct_name #ty_generics #struct_body

        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable for #struct_name #ty_generics #where_clause {
            fn render_to(&self, #buffer_ident: &mut ::hypertext::Buffer) {
                ::hypertext::Renderable::render_to(
                    &#fn_name(
                        #(#field_refs self.#field_names),*
                    ),
                    #buffer_ident
                )
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
