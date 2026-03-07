use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Error, FnArg, Ident, ItemFn, LitBool, Pat, PatType, Path, Token, Type, Visibility,
    parse::Parse, parse_quote,
};

use crate::html::generate::Generator;

pub enum BuilderArg {
    False,
    Path(Path),
}

impl Parse for BuilderArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        if name != "builder" {
            return Err(Error::new(name.span(), "unknown argument"));
        }

        input.parse::<Token![=]>()?;

        let builder = if input.peek(LitBool) {
            let lit_bool = input.parse::<LitBool>()?;
            if lit_bool.value {
                return Err(Error::new(lit_bool.span(), "unexpected `true`"));
            }
            Self::False
        } else {
            Self::Path(input.parse()?)
        };
        Ok(builder)
    }
}

pub struct RenderableArgs {
    visibility: Visibility,
    ident: Option<Ident>,
    builder: Option<BuilderArg>,
    attrs: Option<Vec<Path>>,
}

impl Parse for RenderableArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut visibility = Visibility::Inherited;
        let mut ident = None;
        let mut builder = None;
        let mut attrs: Option<Vec<_>> = None;

        while !input.is_empty() {
            if input.peek(Ident) && input.peek2(Token![=]) {
                let ident = input.fork().parse::<Ident>()?;
                if ident == "attrs" {
                    let _attrs = input.parse::<Ident>()?;

                    input.parse::<Token![=]>()?;

                    let content;
                    syn::bracketed!(content in input);

                    attrs
                        .get_or_insert_default()
                        .extend(content.parse_terminated(Path::parse, Token![,])?);
                } else {
                    builder = Some(input.parse()?);
                }
            } else {
                visibility = input.parse()?;
                if input.peek(Ident) {
                    ident = Some(input.parse()?);
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            visibility,
            ident,
            builder,
            attrs,
        })
    }
}

#[allow(clippy::too_many_lines)]
pub fn generate(args: RenderableArgs, mut fn_item: ItemFn) -> syn::Result<TokenStream> {
    let mut fields = Vec::new();
    let mut field_names = Vec::new();
    let mut field_refs = Vec::new();
    let mut component_attrs = Vec::new();

    let builder = args.builder.or_else(|| {
        if fn_item.sig.inputs.is_empty() {
            None
        } else {
            if args.attrs.is_none() {
                component_attrs = vec![parse_quote!(builder)];
            }

            Some(BuilderArg::Path(parse_quote!(::hypertext::TypedBuilder)))
        }
    });

    component_attrs.extend(args.attrs.unwrap_or_default());

    let vis = if args.visibility == Visibility::Inherited {
        fn_item.vis.clone()
    } else {
        args.visibility
    };

    for input in &mut fn_item.sig.inputs {
        if let FnArg::Typed(PatType { attrs, pat, ty, .. }) = input {
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
                        (&*ty, None)
                    } else {
                        (&ty_ref.elem, Some(ty_ref.and_token))
                    }
                }
                _ => (&*ty, None),
            };

            let field_attrs = attrs
                .extract_if(.., |attr| component_attrs.contains(attr.path()))
                .collect::<Vec<_>>();

            fields.push(quote! {
                #(#field_attrs)*
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

    let mut struct_attrs = fn_item
        .attrs
        .extract_if(.., |attr| {
            attr.path().is_ident("derive") || component_attrs.contains(attr.path())
        })
        .map(|attr| quote!(#attr))
        .collect::<Vec<_>>();

    if let Some(BuilderArg::Path(path)) = builder {
        struct_attrs.push(quote! {
            #[derive(#path)]
        });
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

    let maybe_unit_builder_impl = if fields.is_empty() {
        Some(quote! {
            #[automatically_derived]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #vis fn builder() -> Self {
                    Self
                }

                #vis fn build(self) -> Self {
                    self
                }
            }
        })
    } else {
        None
    };

    let buffer_ident = Generator::buffer_ident();

    let output = quote! {
        #[allow(clippy::needless_lifetimes)]
        #fn_item

        #(#struct_attrs)*
        #vis struct #struct_name #ty_generics #struct_body

        #maybe_unit_builder_impl

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
