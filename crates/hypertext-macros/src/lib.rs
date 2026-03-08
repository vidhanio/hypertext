#![expect(missing_docs)]

mod derive;
mod html;
mod renderable;

use html::{AttributeValue, Many};
use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input};

use self::html::{Document, Maud, Rsx};
use crate::html::generate::{Config, Generate, Semantics};

fn generate<T: Parse + Generate>(config: Config, tokens: TokenStream) -> TokenStream {
    config
        .generate::<T>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn generate_file<T: Parse + Generate>(config: Config, tokens: TokenStream) -> TokenStream {
    config
        .generate_file::<T>(tokens.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

macro_rules! create_variants {
    {
        $($Ty:ty {
            $lazy_move:ident
            $lazy_borrow:ident
            $simple:ident
        })*
    } => {
        $(#[proc_macro]
        pub fn $lazy_move(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config::Lazy(Semantics::Move), tokens)
        }

        #[proc_macro]
        pub fn $lazy_borrow(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config::Lazy(Semantics::Borrow), tokens)
        }

        #[proc_macro]
        pub fn $simple(tokens: TokenStream) -> TokenStream {
            generate::<$Ty>(Config::Simple, tokens)
        })*
    };
}

create_variants! {
    Document<Maud> {
        maud
        maud_borrow
        maud_simple
    }

    Document<Rsx> {
        rsx
        rsx_borrow
        rsx_simple
    }

    Many<AttributeValue> {
        attribute
        attribute_borrow
        attribute_simple
    }
}

// File-based macros: load RSX from external files.

#[proc_macro]
pub fn rsx_file(tokens: TokenStream) -> TokenStream {
    generate_file::<Document<Rsx>>(Config::Lazy(Semantics::Move), tokens)
}

#[proc_macro]
pub fn rsx_file_borrow(tokens: TokenStream) -> TokenStream {
    generate_file::<Document<Rsx>>(Config::Lazy(Semantics::Borrow), tokens)
}

// html! aliases: identical to rsx! variants, avoids Dioxus CLI name collision.
// See https://github.com/vidhanio/hypertext/issues/123.

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream {
    generate::<Document<Rsx>>(Config::Lazy(Semantics::Move), tokens)
}

#[proc_macro]
pub fn html_borrow(tokens: TokenStream) -> TokenStream {
    generate::<Document<Rsx>>(Config::Lazy(Semantics::Borrow), tokens)
}

#[proc_macro]
pub fn html_file(tokens: TokenStream) -> TokenStream {
    generate_file::<Document<Rsx>>(Config::Lazy(Semantics::Move), tokens)
}

#[proc_macro]
pub fn html_file_borrow(tokens: TokenStream) -> TokenStream {
    generate_file::<Document<Rsx>>(Config::Lazy(Semantics::Borrow), tokens)
}

#[proc_macro_derive(Renderable, attributes(maud, rsx, attribute))]
pub fn derive_renderable(input: TokenStream) -> TokenStream {
    derive::renderable(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
    renderable::generate(parse_macro_input!(attr), parse_macro_input!(item))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(DefaultBuilder, attributes(builder))]
pub fn derive_default_builder(input: TokenStream) -> TokenStream {
    derive::default_builder(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
