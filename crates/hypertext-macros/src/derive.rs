use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, spanned::Spanned};

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
